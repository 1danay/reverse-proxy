use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};
use tokio::io::Result;

use crate::models::Backend;

pub async fn handle_connection(
    mut client_socket: TcpStream, 
    backends: Arc<Vec<Backend>>, 
    counter: Arc<AtomicUsize>
) -> Result<()> {
    let mut client_buf = [0; 1024];
    let n = client_socket.read(&mut client_buf).await?;
    if n == 0 { return Ok(()) }

    let current_count = counter.fetch_add(1, Ordering::SeqCst);
    let num_backends = backends.len();
    let start_index = current_count % num_backends;

    let mut selected_backend = None;
    for i in 0..num_backends {
        let check_index = (start_index + i) % num_backends;
        let backend = &backends[check_index];
        
        if backend.is_alive.load(Ordering::SeqCst) {
            selected_backend = Some(backend);
            break;
        }
    }

    let backend = match selected_backend {
        Some(b) => b,
        None => {
            let _ = client_socket.write_all(b"HTTP/1.1 502 Bad Gateway\r\n\r\nAll backends down").await;
            return Ok(());
        }
    };

    let mut backend_socket = TcpStream::connect(&backend.addr).await?;
    backend_socket.write_all(&client_buf[..n]).await?;

    let mut backend_buf = [0; 4096];
    loop {
        let bytes_read = backend_socket.read(&mut backend_buf).await?;
        if bytes_read == 0 { break; }
        client_socket.write_all(&backend_buf[..bytes_read]).await?;
    }

    Ok(())
}