use std::io::Result;

use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream};

pub async fn handle_connection(mut client_socket: TcpStream) -> Result<()> {
  let mut client_buf = [0; 1024];
  let n = client_socket.read(&mut client_buf).await?;
  if n == 0 { return Ok(()) }

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

      client_socket.write_all(&backend_buf[..bytes_read]).await?;
  }

  Ok(())
}