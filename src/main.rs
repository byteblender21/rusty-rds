mod database;
mod network;
mod sql;

use crate::database::parse_query;
use crate::network::is_request_ready;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            let mut request_data = String::new();

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                request_data.push_str(String::from_utf8_lossy(&buf[0..n]).as_ref());

                if is_request_ready(&buf[0..n]) {
                    if let Err(e) = socket.write_all("stop".as_bytes()).await {
                        eprintln!("failed to write to socket; err ) {:?}", e);
                        return;
                    }

                    parse_query(request_data.as_ref());
                } else {
                    // Write the data back
                    if let Err(e) = socket.write_all(&buf[0..n]).await {
                        eprintln!("failed to write to socket; err = {:?}", e);
                        return;
                    }
                }
            }
        });
    }
}
