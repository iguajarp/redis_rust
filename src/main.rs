use std::{
    io::{self, Result, Write, Read},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379")?;
    listener.set_nonblocking(true).expect("failed to set non-blocking tcp_listener");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                thread::spawn(move || {
                    handle_request(s);
                });
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
