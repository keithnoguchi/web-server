use std::error::Error;
use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};
use std::result;

use tracing::{info, instrument};

type Result<T> = result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;

    for stream in listener.incoming() {
        let req = parse_request(stream?)?;
        info!(?req);
    }
    Ok(())
}

#[instrument(skip(stream), ret, err)]
fn parse_request(mut stream: TcpStream) -> Result<http::Request<Vec<u8>>> {
    let mut reader = BufReader::new(&mut stream).lines();

    // request line.
    let req_line = reader.next().ok_or("missing request line")??;
    let req_line: Vec<_> = req_line.split_whitespace().collect();
    let mut req = http::Request::default();
    *req.method_mut() = http::Method::try_from(req_line[0])?;
    *req.uri_mut() = http::Uri::try_from(req_line[1])?;
    // no check for version and default to HTTP/1.1.

    // headers.
    let headers = req.headers_mut();
    for line in reader {
        let line = line?;
        if line.is_empty() {
            break;
        }
        let header: Vec<_> = line.split(':').collect();
        let key = http::HeaderName::try_from(header[0].trim())?;
        let value = http::HeaderValue::from_str(header[1].trim())?;
        headers.append(key, value);
    }

    /*
    // body...
    *req.body_mut() = reader
        .flat_map(|line| line.unwrap().into_bytes())
        .collect();
    */

    Ok(req)
}
