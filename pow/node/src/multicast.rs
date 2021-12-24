use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket};

struct Addresses {
    collection: SocketAddr[]
}

impl Addresses {
    fn add(self, address: SocketAddr) {
        self.collection.push(address);
    }
}

#[cfg(windows)]
pub fn bind(multicast: Ipv4Addr, local: Ipv4Addr) -> Result<UdpSocket> {
    socket
        .join_multicast_v4(&IPV4_MULTICAST, &IPV4_LOCAL)
        .expect("Error Joining Multicast");
    println!("Is Multicast: {}", IPV4_MULTICAST.is_multicast());
    socket
}

#[cfg(unix)]
pub fn bind() {}

fn new_socket(ip: IpAddr, port: u16) -> UdpSocket {
    UdpSocket::bind(&addresses[..]).unwrap()
}

fn join() {}
