use std::{net::{TcpListener, TcpStream}, io::{Result, Read, Write}};


fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379")?;

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_request(stream);
    }

    Ok(())
}

fn handle_request(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let ping = b"$4\r\nping\r\n";

    stream.read(&mut buffer).unwrap();

    if buffer.starts_with(ping) {
        stream.write(b"+PONG\r\n").unwrap();
        stream.flush().unwrap();
    }
    
}