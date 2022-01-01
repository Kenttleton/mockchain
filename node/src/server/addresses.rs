use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

#[derive(Copy, Clone)]
pub struct LocalAddresses {
    pub v4: Ipv4Addr,
    pub v6: Ipv6Addr,
}

const LOCAL_ADDRESSES: LocalAddresses = LocalAddresses {
    v4: Ipv4Addr::new(0, 0, 0, 0),
    v6: Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0),
};

#[derive(Copy, Clone)]
pub struct Addresses {
    pub v4: Ipv4Addr,
    pub v6: Ipv6Addr,
    pub port: u16,
    pub local: LocalAddresses,
}

impl Addresses {
    pub fn new(v4_address: Ipv4Addr, v6_address: Ipv6Addr, port: u16) -> Self {
        Addresses {
            v4: v4_address,
            v6: v6_address,
            port,
            local: LOCAL_ADDRESSES,
        }
    }
}

//IpV4 for now
#[derive(Copy, Clone)]
pub struct Destination {
    ip: [u8; 4],
    port: u16,
}

impl Destination {
    pub fn to_socket_addr(self) -> SocketAddr {
        SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(
                self.ip[0], self.ip[1], self.ip[2], self.ip[3],
            )),
            self.port,
        )
    }
}

pub const NODE: Destination = Destination {
    ip: [192, 168, 56, 1],
    port: 51404,
};
