use anyhow::Result;
use bytes::BytesMut;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};


mod cache;

mod resp;

#[tokio::main]
async fn main() -> Result<()> {
    let mut listener = TcpListener::bind("127.0.0.1:6379").await?;

    loop {
        let incoming = listener.accept().await;
        match incoming {
            Ok((stream, _)) => {
                println!("accepted new connection");
                tokio::spawn(async move {
                    handle_connection(stream).await.unwrap();
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

}

async fn handle_connection(stream: TcpStream) -> Result<()> {
    let mut conn = resp::RespConnection::new(stream);

    loop {
        let value = conn.read_value().await?;

        if let Some(value) = value {
            let (command, args) = value.to_command()?; // If string command, return early, then panic at parent fragment
            let response = match command.to_ascii_lowercase().as_ref() {
                "ping" => resp::Value::SimpleString("PONG".to_string()),
                "echo" => args.first().unwrap().clone(),
                /*
                    TODO: implement cache system
                    1) if command is set, check that args are len 2.
                        1.1) if not, responde with error msg
                    2) get singleton cache.
                    3) create method to get value if exists or None at cache.
                    4) save key - value in cache singleton
                    5) return Value::SingleString("OK") or Value::SingleString("Error setting value")
                    6) implement "GET" command using cache.getValue()
                    
                */
                "SET" => resp::Value::SimpleString("OK".to_string()),
                _ => resp::Value::Error(format!("command not implemented: {}", command)),
            };

            conn.write_value(response).await?;
        } else {
            break;
        }
    }

    Ok(())
}
