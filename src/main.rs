use std::{net::TcpListener, io::Result};


fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379")?;
    match listener.accept() {
        Ok((_socket, addr)) => println!("new client: {addr:?}"),
        Err(e) => println!("couldn't get client: {e:?}"),
    }
    
    Ok(())
}
