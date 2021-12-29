use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};

pub struct LocalAddresses {
    pub v4: Ipv4Addr,
    pub v6: Ipv6Addr,
}

const LOCAL_ADDRESSES: LocalAddresses = LocalAddresses {
    v4: Ipv4Addr::new(0, 0, 0, 0),
    v6: Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0),
};

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
