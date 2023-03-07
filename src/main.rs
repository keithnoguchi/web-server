use std::error::Error;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::result;

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;

    for stream in listener.incoming() {
        handle_connection(stream?)?;
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let mut reader = BufReader::new(&mut stream).lines();

    // request line.
    let req_line = reader.next().ok_or("missing request line")??;
    println!("{req_line:#?}");

    // headers.
    let mut headers = vec![];
    for line in reader {
        let line = line?;
        if line.is_empty() {
            break;
        }
        headers.push(line);
    }
    println!("{headers:#?}");

    Ok(())
}
