pub mod addresses;
mod sockets;
use crate::server::addresses::Addresses;

pub fn listener(addresses: Addresses, func: fn([u8; 1000000]) -> [u8; 1000000]) {
    let socket_v4 = &sockets::bind_v4(&addresses);
    sockets::join_v4(socket_v4, &addresses);
    sockets::listen_v4(socket_v4, func);
}
