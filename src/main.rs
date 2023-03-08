use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::result;

type Result<T> = result::Result<T, Box<dyn Error>>;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;

    for s in listener.incoming() {
        let s = s?;
        if let Err(e) = handle_connection(s) {
            dbg!(e);
        }
    }

    Ok(())
}

fn handle_connection(mut s: TcpStream) -> Result<()> {
    // request.
    let req_line = BufReader::new(&mut s)
        .lines()
        .next()
        .unwrap()
        .unwrap();

    dbg!(&req_line);

    match req_line.as_str() {
        "GET / HTTP/1.1" => {
            let status_line = "HTTP/1.1 200 OK";
            let content = fs::read_to_string("index.html")?;
            let length = content.len();
            let resp = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
            dbg!(&resp);

            s.write_all(resp.as_bytes())?;
        }
        _ => {
            let status_line = "HTTP/1.1 404 Not found";
            let content = fs::read_to_string("404.html")?;
            let length = content.len();
            let resp = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
            dbg!(&resp);

            s.write_all(resp.as_bytes())?;
        }
    }

    Ok(())
}
