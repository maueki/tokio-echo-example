use tokio::prelude::*;
use futures::future::{loop_fn, Loop};
use tokio::net::UdpSocket;
use std::net::SocketAddr;

fn main() {
    let addr = "127.0.0.1:9000".to_string().parse::<SocketAddr>().unwrap();
    let socket = UdpSocket::bind(&addr).unwrap();
    println!("Listening on: {}", socket.local_addr().unwrap());

    let server = loop_fn(socket, |socket| {
        let buf = vec![0; 1024];
        socket.recv_dgram(buf)
            .and_then(|(socket, buf, _size, peer)| socket.send_dgram(buf, &peer))
            .and_then(|(socket, _)| Ok(Loop::Continue(socket)))
    });

    tokio::run(server.map_err(|_| ()));
}
