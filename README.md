# web-server

[![CI](https://github.com/keithnoguchi/web-server/actions/workflows/ci.yml/badge.svg)](
https://github.com/keithnoguchi/web-server/actions)

[web server]: https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html
[the book]: https://doc.rust-lang.org/book

A [web server], as in [the book].

## Examples

```
use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::result;
use std::sync::mpsc::{sync_channel, SyncSender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

pub type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

type Job = Box<dyn FnOnce() -> Result<()> + Send + Sync + 'static>;

#[derive(Debug)]
pub struct ThreadPool {
    tx: Option<SyncSender<Job>>,
    workers: Vec<JoinHandle<()>>,
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
    pub fn new(size: usize) -> Self {
        let (tx, rx) = sync_channel::<Job>(size);
        let tx = Some(tx);
        let rx = Arc::new(Mutex::new(rx));
        let mut workers = vec![];
        for _ in 0..size {
            let rx = rx.clone();
            workers.push(thread::spawn(move || loop {
                let msg = rx.lock().unwrap().recv();
                match msg {
                    Err(_) => return,
                    Ok(job) => {
                        if let Err(e) = job() {
                            dbg!(e);
                        }
                    }
                }
            }));
        }
        Self { tx, workers }
    }

    pub fn execute<F>(&self, f: F) -> Result<()>
    where
        F: FnOnce() -> Result<()> + Send + Sync + 'static,
    {
        self.tx.as_ref().unwrap().send(Box::new(f))?;
        Ok(())
    }
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("localhost:3000")?;
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(5) {
        let stream = stream?;
        if let Err(e) = pool.execute(move || handler(stream)) {
            dbg!(e);
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
```

Happy Hacking!
