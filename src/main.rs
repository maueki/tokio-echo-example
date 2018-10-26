use tokio::prelude::*;
use tokio::net::UdpSocket;
use std::net::SocketAddr;

use futures::try_ready;

struct EchoServer {
    socket: UdpSocket,
    buf: Vec<u8>,
    to_send: Option<(usize, SocketAddr)>,
}

impl EchoServer {
    fn new(socket: UdpSocket) -> EchoServer {
        EchoServer{socket: socket, buf: vec![0; 1024], to_send: None}
    }
}

impl Future for EchoServer {
    type Item = ();
    type Error = std::io::Error;

    fn poll(&mut self) -> Poll<(), std::io::Error> {
        loop {
            if self.to_send == None {
                self.to_send = Some(try_ready!(self.socket.poll_recv_from(&mut self.buf)));
            }

            if let Some((size, peer)) = self.to_send {
                let amt = try_ready!(self.socket.poll_send_to(&self.buf[..size], &peer));
                println!("Echoed {}/{} bytes to {}", amt, size, peer);
                self.to_send = None;
            }
        }
    }
}

fn main() {
    let addr = "127.0.0.1:9000".to_string().parse::<SocketAddr>().unwrap();
    let socket = UdpSocket::bind(&addr).unwrap();
    println!("Listening on: {}", socket.local_addr().unwrap());

    let server = EchoServer::new(socket);

    tokio::run(server.map_err(|_| ()));
}
