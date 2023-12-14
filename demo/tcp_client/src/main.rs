use std::{io::Write, net::Shutdown};

use wasmedge_wasi_socket::TcpStream;

fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or("1234".to_string());
    println!("connect to 127.0.0.1:{}", port);
    let mut stream = TcpStream::connect(format!("127.0.0.1:{}", port))?;
    println!("local address {}", stream.local_addr().unwrap());
    println!("peer address {}", stream.peer_addr().unwrap());
    let data = b"hello";
    println!("sending hello message {} ...", data.len());
    let _ = stream.write(data);
    stream.shutdown(Shutdown::Both)?;
    Ok(())
}
