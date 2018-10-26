#![feature(pin, await_macro, async_await, futures_api)]
use tokio::await;
use tokio::net::UdpSocket;
use std::net::SocketAddr;

fn main() {
    let addr = "127.0.0.1:9000".to_string().parse::<SocketAddr>().unwrap();
    let mut socket = UdpSocket::bind(&addr).unwrap();
    println!("Listening on: {}", socket.local_addr().unwrap());

    tokio::run_async(async move {
        loop {
            let buf = vec![0; 1024];
            let (sock, buf, size, peer) = await!(socket.recv_dgram(buf)).unwrap();
            let (sock, _) = await!(sock.send_dgram(buf, &peer)).unwrap();
            socket = sock;
            println!("Echoed {} to {}", size, peer);
        }
    });
}
