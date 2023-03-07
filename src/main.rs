use std::net::TcpListener;

fn main() -> eyre::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;

    for stream in listener.incoming() {
        let stream = stream?;
        println!("{stream:#?}");
    }
    Ok(())
}
