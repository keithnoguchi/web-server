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
        handle_connection(s)?;
    }

    Ok(())
}

fn handle_connection(mut s: TcpStream) -> Result<()> {
    // request.
    let req: Vec<_> = BufReader::new(&mut s)
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    dbg!(req);

    // response.
    let status_line = "HTTP/1.1 200 OK";
    let body = fs::read_to_string("index.html")?;
    let body_len = body.len();
    let resp = format!("{status_line}\r\nContent-Length: {body_len}\r\n\r\n{body}");

    dbg!(&resp);

    s.write_all(resp.as_bytes())?;

    Ok(())
}
