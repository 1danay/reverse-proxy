use std::{
    io::Result,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub async fn handle_connection(
    mut client_socket: TcpStream,
    backends: Arc<Vec<String>>,
    counter: Arc<AtomicUsize>,
) -> Result<()> {
    let mut client_buf = [0; 1024];
    let n = client_socket.read(&mut client_buf).await?;
    if n == 0 {
        return Ok(());
    }

    println!("Recieved: \n {}", String::from_utf8_lossy(&client_buf[..n]));

    let current_count = counter.fetch_add(1, Ordering::SeqCst);
    println!("CURRENT_COUNT: {}", current_count);
    let index = current_count % backends.len();
    let backend_addr = &backends[index];

    println!(
        "--> Перенаправляем запрос на бэкенд [{}]: {}",
        index, backend_addr
    );

    let mut backend_socket = TcpStream::connect(backend_addr).await?;

    backend_socket.write_all(&client_buf[..n]).await?;

    let mut backend_buf = [0; 4096];

    loop {
        let bytes_read = backend_socket.read(&mut backend_buf).await?;

        if bytes_read == 0 {
            break;
        }

        println!(
            "=== Ответ от сервера {} ===\n{}",
            backend_addr,
            String::from_utf8_lossy(&backend_buf[..bytes_read])
        );

        client_socket.write_all(&backend_buf[..bytes_read]).await?;
    }

    Ok(())
}
