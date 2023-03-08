use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::result;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;
    let pool = ThreadPool::new(4);

    // only take 5 sessions.
    for s in listener.incoming().take(5) {
        let s = s?;
        pool.execute(|| handle_connection(s));
    }

    Ok(())
}

struct ThreadPool {
    workers: Vec<JoinHandle<()>>,
    tx: Option<SyncSender<Box<dyn FnOnce() -> Result<()> + Send + 'static>>>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        if let Some(tx) = self.tx.take() {
            drop(tx);
            for worker in self.workers.drain(..) {
                if let Err(e) = worker.join() {
                    dbg!(e);
                }
            }
        }
    }
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        let (tx, rx) = sync_channel::<Box<dyn FnOnce() -> Result<()> + Send + 'static>>(size);
        let tx = Some(tx);
        let rx = Arc::new(Mutex::new(rx));
        let workers: Vec<_> = (0..size)
            .map(|_| {
                let rx = rx.clone();
                thread::spawn(move || loop {
                    let f = rx.lock().unwrap().recv();
                    match f {
                        Ok(f) => {
                            if let Err(e) = f() {
                                dbg!(e);
                            }
                        }
                        Err(e) => {
                            dbg!(e);
                            return;
                        }
                    }
                })
            })
            .collect();
        Self { workers, tx }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() -> Result<()> + Send + 'static,
    {
        if let Some(tx) = self.tx.as_ref() {
            if let Err(e) = tx.send(Box::new(f)) {
                dbg!(e);
            }
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
