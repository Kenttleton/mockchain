use crate::server::addresses::Addresses;
use std::net::{SocketAddr, UdpSocket};

pub fn listen_v4(socket: &UdpSocket, func: fn([u8; 1000000]) -> [u8; 1000000]) {
    loop {
        // 1 MB buffer per Bitcoin spec for a block size
        let mut buf = [0; 1000000];
        let (amt, src) = socket.recv_from(&mut buf).expect("No Data Received");
        let res = func(buf);
        println!(
            "Source: {}\nAmount: {}\nResponse: {}",
            &src,
            &amt,
            u8_to_string(&res)
        );
        socket.send_to(&res, &src).expect("Error Sending Response");
    }
}

fn u8_to_string(u8_arr: &[u8; 1000000]) -> String {
    let mut message = String::new();
    for x in u8_arr.iter() {
        message.push(*x as char);
    }
    message
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
pub fn bind_v6(addresses: &Addresses) -> UdpSocket {
    UdpSocket::bind(SocketAddr::from((addresses.local.v6, addresses.port)))
        .expect("Failure to bind IPv6 on Windows")
}

#[cfg(unix)]
pub fn bind_v6(addresses: &Addresses) -> UdpSocket {
    UdpSocket::bind(SocketAddr::from((addresses.v6, addresses.port)))
        .expect("Failure to bind IPv6 on Unix")
}

#[cfg(windows)]
pub fn join_v4(socket: &UdpSocket, addresses: &Addresses) {
    socket
        .join_multicast_v4(&addresses.v4, &addresses.local.v4)
        .expect("Error Joining V4 Multicast");
}

#[cfg(unix)]
pub fn join_v4(socket: &UdpSocket, addresses: &Addresses) {}

#[cfg(windows)]
pub fn join_v6(socket: &UdpSocket, addresses: &Addresses) {
    socket
        .join_multicast_v6(&addresses.v6, 0)
        .expect("Error Joining V6 Multicast");
}

#[cfg(unix)]
pub fn join_v6(socket: &UdpSocket, addresses: &Addresses) {}
