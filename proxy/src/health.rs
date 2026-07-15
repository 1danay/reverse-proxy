use std::{sync::{Arc, atomic::Ordering}, time::Duration};

use tokio::{net::TcpStream, time::{self, timeout}};

use crate::models::Backend;

pub fn start_health_checker(backends: Arc<Vec<Backend>>) {
  tokio::spawn(async move {
    let mut ticker = time::interval(Duration::from_secs(5));
    loop {
      ticker.tick().await;
      println!("--- [Health Check] ---");

      for backend in backends.iter() {
        let connect_future = TcpStream::connect(&backend.addr);

        match timeout(Duration::from_secs(1), connect_future).await {
          Ok(Ok(_stream)) => {
            println!("Backend {} - is_alive", backend.addr);
            backend.is_alive.store(true, Ordering::SeqCst);
          }
          _ => {
            println!("Backend {} - not_alive", backend.addr);
            backend.is_alive.store(false, Ordering::SeqCst);
          }
        }
      }
    }
  });
}