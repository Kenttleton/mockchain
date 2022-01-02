use crate::server::addresses::{Addresses, Destination};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::str;

pub fn listen_v4(socket: &UdpSocket, func: fn(&str) -> (String, Destination)) {
    let (_src, block) = receive(socket);
    let (msg, dst) = func(block.as_str());
    send_to(socket, &msg, &dst.to_socket_addr());
    listen_v4(socket, func);
}

fn receive(socket: &UdpSocket) -> (SocketAddr, String) {
    let mut buf = [0; 1500];
    let mut amt: usize = 0;
    let mut src: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);
    match socket.recv_from(&mut buf) {
        Ok((a, s)) => {
            amt = a;
            src = s;
        }
        Err(e) => println!("{}", e),
    }
    let msg: &str = str::from_utf8(&buf[..amt]).unwrap();
    (src, String::from(msg))
}

fn send_to(socket: &UdpSocket, res: &str, dst: &SocketAddr) {
    socket
        .send_to(res.as_bytes(), &dst)
        .expect("Error Sending Response");
}

#[cfg(windows)]
pub fn bind_v4(addresses: &Addresses) -> UdpSocket {
    UdpSocket::bind(&SocketAddr::from((addresses.local.v4, addresses.port)))
        .expect("Failure to bind IPv4 on Windows")
}

#[cfg(unix)]
pub fn bind_v4(addresses: &Addresses) -> UdpSocket {
    UdpSocket::bind(SocketAddr::from((addresses.v4, addresses.port)))
        .expect("Failure to bind IPv4 on Unix")
}

#[cfg(windows)]
pub fn join_v4(socket: &UdpSocket, addresses: &Addresses) {
    socket
        .join_multicast_v4(&addresses.v4, &addresses.local.v4)
        .expect("Error Joining V4 Multicast");
}

#[cfg(unix)]
pub fn join_v4(socket: &UdpSocket, addresses: &Addresses) {}
