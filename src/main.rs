//! A web server

use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::result;

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

fn main() -> Result<()> {
    let listener = TcpListener::bind("localhost:3000")?;

    for stream in listener.incoming() {
        let stream = stream?;
        if let Err(e) = handler(stream) {
            eprintln!("{e}");
        }
    }

    Ok(())
}

fn handler(mut s: TcpStream) -> Result<()> {
    let reader = BufReader::new(&mut s);
    let req_line = reader.lines().next().ok_or("invalid request line")??;

    let (status, file) = match req_line.as_ref() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let content = fs::read_to_string(file)?;
    let length = content.len();
    let resp = format!("{status}\r\nContent-Length: {length}\r\n\r\n{content}");

    s.write_all(resp.as_bytes())?;

    Ok(())
}
