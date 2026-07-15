use std::{
    io::Result,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize},
    },
};

use tokio::net::TcpListener;

use crate::{
    health::start_health_checker, models::Backend, proxy::handle_connection,
    rate_limiting::start_rate_limiter_reset,
};

mod health;
mod models;
mod proxy;
mod rate_limiting;

#[tokio::main]
async fn main() -> Result<()> {
    let backends: Vec<Backend> = vec![
        Backend {
            addr: "127.0.0.1:9000".to_string(),
            is_alive: AtomicBool::new(true),
        },
        Backend {
            addr: "127.0.0.1:9001".to_string(),
            is_alive: AtomicBool::new(true),
        },
        Backend {
            addr: "127.0.0.1:9002".to_string(),
            is_alive: AtomicBool::new(true),
        },
    ];

    let backends = Arc::new(backends);

    let counter = Arc::new(AtomicUsize::new(0));
    let rate_limit_counter = Arc::new(AtomicUsize::new(0));

    start_rate_limiter_reset(Arc::clone(&rate_limit_counter), 10);

    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    println!("Proxy started on port 8081");

    start_health_checker(Arc::clone(&backends));

    loop {
        let (socket, _addr) = listener.accept().await?;

        let backends_clone = Arc::clone(&backends);
        let counter_clone = Arc::clone(&counter);

        let rate_limit_clone = Arc::clone(&rate_limit_counter);

        tokio::spawn(async move {
            if let Err(e) =
                handle_connection(socket, backends_clone, counter_clone, rate_limit_clone).await
            {
                eprintln!("Error client handling: {}", e)
            }
        });
    }
}
