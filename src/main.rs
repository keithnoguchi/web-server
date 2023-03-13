//! A web server

use std::error::Error;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::result;

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

fn main() -> Result<()> {
    let listener = TcpListener::bind("localhost:3000")?;

    for stream in listener.incoming() {
        let stream = stream?;
        handler(stream);
    }

    Ok(())
}

fn handler(mut s: TcpStream) -> Result<()> {
    let reader = BufReader::new(&mut s);
    let req_line = reader
        .lines()
        .next()
        .ok_or("invalid request line")??;

    println!("{req_line}");

    Ok(())
}
