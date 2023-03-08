use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::result;
use std::thread;
use std::time::Duration;

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;
    let pool = ThreadPool::new(4);

    for s in listener.incoming() {
        let s = s?;
        pool.execute(|| handle_connection(s));
    }

    Ok(())
}

struct ThreadPool<R: Send + 'static> {
    _workers: Vec<thread::JoinHandle<Result<R>>>,
}

impl<R: Send + 'static> ThreadPool<R> {
    fn new(size: usize) -> Self {
        assert!(size != 0);
        let worker = || loop {
            thread::sleep(Duration::from_secs(1));
        };
        let _workers: Vec<_> = (0..size).map(|_| thread::spawn(worker)).collect();

        Self { _workers }
    }

    fn execute<F>(&self, handler: F)
    where
        F: FnOnce() -> Result<R>,
    {
        if let Err(e) = handler() {
            dbg!(e);
        }
    }
}

fn handle_connection(mut s: TcpStream) -> Result<()> {
    // A request.
    let req_line = BufReader::new(&mut s)
        .lines()
        .next()
        .ok_or("invalid HTTP request line")??;

    dbg!(&req_line);

    // A response.
    let (status, file) = match req_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        _ => ("HTTP/1.1 404 Not Found", "404.html"),
    };
    let content = fs::read_to_string(file)?;
    let len = content.len();
    let resp = format!("{status}\r\nContent-Length: {len}\r\n\r\n{content}");
    s.write_all(resp.as_bytes())?;

    Ok(())
}
