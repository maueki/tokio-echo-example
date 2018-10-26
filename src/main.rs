use tokio::prelude::*;
use tokio::net::UdpSocket;
use std::net::SocketAddr;

use futures::try_ready;

struct EchoServer {
    socket: UdpSocket,
    buf: Vec<u8>,
}

impl EchoServer {
    fn new(socket: UdpSocket) -> EchoServer {
        EchoServer{socket: socket, buf: vec![0; 1024]}
    }
}

impl Future for EchoServer {
    type Item = ();
    type Error = std::io::Error;

    fn poll(&mut self) -> Poll<(), std::io::Error> {
        let _ = try_ready!(self.socket.poll_recv_from(&mut self.buf));
        println!("connect");
        Ok(Async::Ready(()))
    }
}

fn main() {
    let addr = "127.0.0.1:9000".to_string().parse::<SocketAddr>().unwrap();
    let socket = UdpSocket::bind(&addr).unwrap();
    println!("Listening on: {}", socket.local_addr().unwrap());

    let server = EchoServer::new(socket);

    tokio::run(server.map_err(|_| ()));
}
