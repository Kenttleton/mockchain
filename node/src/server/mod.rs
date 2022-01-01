pub mod addresses;
mod sockets;
use crate::server::addresses::{Addresses, Destination};

pub fn listener_v4(addresses: Addresses, func: fn(&str) -> (String, Destination)) {
    let socket_v4 = &sockets::bind_v4(&addresses);
    sockets::join_v4(socket_v4, &addresses);
    sockets::listen_v4(socket_v4, func);
}
