use std::{
    io::{Read, Result, Write},
    net::{TcpListener, TcpStream},
    thread,
    str::{self, from_utf8}, string
};

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379")?;
    listener
        .set_nonblocking(true)
        .expect("failed to set non-blocking tcp_listener");

    for stream in listener.incoming() {
        match stream {
            Ok(s) => {
                thread::spawn(move || {
                    handle_request(s);
                });
            }
            Err(_) => {}
        }
    }

    Ok(())
}

fn handle_request(mut stream: TcpStream) {
    let mut buf = [0; 512];

    loop {
        match stream.read(&mut buf) {
            Ok(_size) => {
                if buf.starts_with(b"*2\r\n$4\r\necho") {
                    let echo = echo_string(&mut buf);
                    stream.write(format!("+{echo}\r\n").as_bytes()).unwrap();
                } else if buf.starts_with(b"*1\r\n$4\r\nping\r\n") {
                    stream.write(b"+PONG\r\n").unwrap();
                } else {
                    stream.write(b"+Invalid request\r\n").unwrap();
                }
                stream.flush().unwrap();
            }
            Err(_) => {
                break;
            }
        }
    }
}

fn echo_string(buf: &mut [u8; 512]) -> &str {
    let string_command = from_utf8(buf).unwrap();
    let splitted: Vec<&str> = string_command.split("\r\n").collect();
    splitted[4]
}