use std::{
    fs,
    io::{
        BufReader,
        prelude::*,
    },
    net::{
        TcpListener,
        TcpStream,
    },
};
use std::io::{stdin, stdout};
use anyhow::Result;
use rayon::{ThreadPool, ThreadPoolBuilder};


fn main() -> Result<()> {
    stdout().write("Enter IP: ".as_bytes())?;
    stdout().flush()?;
    let mut ip: String = String::new();
    stdin().read_line(&mut ip)?;
    let listener: TcpListener = TcpListener::bind(ip.trim())?;
    let pool: ThreadPool = ThreadPoolBuilder::new().num_threads(4).build()?;
    for stream in listener.incoming() {
        let stream: TcpStream = stream?;

        pool.spawn(|| {
            handle_connection(stream).unwrap();
        })
    }

    return Ok(());
}


fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let request_line: String = buf_reader.lines().next().unwrap()?;

    let (status_line, filename): (&str, &str) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "html/index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "html/404.html")
    };

    let contents: String = fs::read_to_string(filename)?;
    let length: usize = contents.len();

    let response: String = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes())?;
    return Ok(())
}
