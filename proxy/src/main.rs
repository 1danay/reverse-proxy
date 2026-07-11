use std::io::Result;

use tokio::net::TcpListener;

use crate::proxy::handle_connection;

mod proxy;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    println!("Proxy started on port 8081");

    loop {
        let (socket, _addr) = listener.accept().await?;

        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
                eprintln!("Error client handling: {}", e)
            }
        });
    }
}
