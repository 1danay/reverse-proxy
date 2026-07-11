use std::io::Result;

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8081").await?;
    println!("Proxy started on port 8081");

    loop {
        let (mut socket, _addr) = listener.accept().await?;

        let mut client_buf = [0; 1024];
        let n = socket.read(&mut client_buf).await?;
        if n == 0 { continue; }

        println!("Recieved: \n {}", String::from_utf8_lossy(&client_buf[..n]));

        let backend_addr = "127.0.0.1:9000";
        let mut backend_socket = TcpStream::connect(backend_addr).await?;

        backend_socket.write_all(&client_buf[..n]).await?;

        let mut backend_buf = [0; 4096];

        loop {
            let bytes_read = backend_socket.read(&mut backend_buf).await?;

            if bytes_read == 0 {
                break;
            }

            socket.write_all(&backend_buf[..bytes_read]).await?;
        }
    }
}

