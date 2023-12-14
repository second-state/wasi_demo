use std::net::SocketAddr;

use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

async fn handle_client((mut stream, addr): (TcpStream, SocketAddr)) -> std::io::Result<()> {
    let local_addr = stream.local_addr()?;
    println!("{} <-> {}", addr.to_string(), local_addr);
    let mut buf = String::new();

    let mut reader = BufReader::new(&mut stream);
    reader.read_line(&mut buf).await?;
    println!("get message: {}", buf);
    println!("sendback reversed message...");
    stream
        .write(&buf.chars().rev().collect::<String>().into_bytes())
        .await?;

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let port = std::env::var("PORT").unwrap_or("1234".to_string());
    println!("listening at 127.0.0.1:{}", port);
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();

    for _ in 0..10 {
        tokio::spawn(handle_client(listener.accept().await.unwrap()));
    }
}
