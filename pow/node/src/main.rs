use cryptography::RSA;
use ctrlc;
use std::net::{Ipv4Addr, Ipv6Addr};
mod blockchain;
mod server;

pub const PORT: u16 = 7645;
pub static IPV4: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 123);
pub static IPV6: Ipv6Addr = Ipv6Addr::new(0xFF02, 0, 0, 0, 0, 0, 0, 0x0123);

fn main() {
    // std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    // env_logger::init();
    ctrlc::set_handler(move || std::process::exit(0)).expect("Error setting Ctrl-C handler");
    let cert = RSA::new(4096);
    let addresses = server::addresses::Addresses::new(IPV4, IPV6, PORT);
    server::listener(addresses, blockchain::handler);
}
