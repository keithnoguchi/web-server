use std::error::Error;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::result;

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;

    for stream in listener.incoming() {
        let mut stream = stream?;
        handle_connection(&mut stream)?;
    }

    Ok(())
}

fn handle_connection(s: &mut TcpStream) -> Result<()> {
    let req: Vec<_> = BufReader::new(s)
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{:#?}", req);

    Ok(())
}
