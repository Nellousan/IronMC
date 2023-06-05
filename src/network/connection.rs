use std::net::SocketAddr;
use tokio::net::TcpStream;

#[derive(Debug)]
pub struct Connection {
    socket: TcpStream,
    address: SocketAddr,
}
