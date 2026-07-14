use std::{
    io::Result,
    sync::{Arc, atomic::AtomicUsize},
};

use tokio::net::TcpListener;

use crate::proxy::handle_connection;

mod proxy;

#[tokio::main]
async fn main() -> Result<()> {
    let backends_addr = vec![
        "localhost:9000".to_string(),
        "localhost:9001".to_string(),
        "localhost:9002".to_string(),
    ];

    let backends = Arc::new(backends_addr);

    let counter = Arc::new(AtomicUsize::new(0));

    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    println!("Proxy started on port 8081");

    loop {
        let (socket, _addr) = listener.accept().await?;

        let backends_clone = Arc::clone(&backends);
        let counter_clone = Arc::clone(&counter);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket, backends_clone, counter_clone).await {
                eprintln!("Error client handling: {}", e)
            }
        });
    }
}
