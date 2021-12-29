use crate::server::addresses::Addresses;
use std::net::{SocketAddr, UdpSocket};

pub fn listen_v4(socket: &UdpSocket, func: fn([u8; 1000000]) -> [u8; 1000000]) {
    loop {
        // 1 MB buffer per Bitcoin spec for a block size
        let mut buf = [0; 1000000];
        let (amt, src) = socket.recv_from(&mut buf).unwrap();
        socket
            .send_to(&func(buf), &src)
            .expect("Error Sending Response");
        println!("Source: {}\nAmount: {}", &src, &amt);
    }
}

#[cfg(windows)]
pub fn bind_v4(addresses: &Addresses) -> UdpSocket {
    UdpSocket::bind(&SocketAddr::from((addresses.local.v4, addresses.port))).unwrap()
}

#[cfg(unix)]
pub fn bind_v4(addresses: &Addresses) -> UdpSocket {
    UdpSocket::bind(SocketAddr::from((addresses.v4, addresses.port))).unwrap()
}

#[cfg(windows)]
pub fn bind_v6(addresses: &Addresses) -> UdpSocket {
    UdpSocket::bind(SocketAddr::from((addresses.local.v6, addresses.port))).unwrap()
}

#[cfg(unix)]
pub fn bind_v6(addresses: &Addresses) -> UdpSocket {
    UdpSocket::bind(SocketAddr::from((addresses.v6, addresses.port))).unwrap()
}

#[cfg(windows)]
pub fn join_v4(socket: &UdpSocket, addresses: &Addresses) {
    socket
        .join_multicast_v4(&addresses.v4, &addresses.local.v4)
        .expect("Error Joining V4 Multicast");
}

#[cfg(unix)]
pub fn join_v4(socket: &UdpSocket) {}

#[cfg(windows)]
pub fn join_v6(socket: &UdpSocket, addresses: &Addresses) {
    socket
        .join_multicast_v6(&addresses.v6, 0)
        .expect("Error Joining V6 Multicast");
}

#[cfg(unix)]
pub fn join_v6(socket: &UdpSocket) {}
