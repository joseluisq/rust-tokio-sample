use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::TcpStream;
use tokio::prelude::*;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6142);
    let mut stream = match TcpStream::connect(addr).await {
        Err(e) => {
            eprintln!("{:?}", e);
            std::process::exit(1);
        }
        Ok(s) => s,
    };

    let result = stream.write(b"Hello, world\n").await;

    println!("wrote to stream! [success={:?}]", result.is_ok());
}
