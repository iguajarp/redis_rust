use std::{
    io::{self, Result, Write, Read},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379")?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut s) => {
                handle_request(s);
            },
            Err(_) => {
                println!("error stablishing connection");
            }
        }
    }

    Ok(())
}

fn handle_request(mut stream: TcpStream) {
    let mut buf = [0; 512];

    loop {
        match stream.read(&mut buf) {
            Ok(_size) => {
                stream.write(b"+PONG\r\n").unwrap();
                stream.flush().unwrap();
            },
            Err(_) => {
                break;
            }
        }
    }

}
