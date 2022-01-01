//use cryptography::RSA;
use ctrlc;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::thread;
mod blockchain;
mod server;

pub const PORT: u16 = 7645;
pub static IPV4: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 123);
pub static IPV6: Ipv6Addr = Ipv6Addr::new(0xFF02, 0, 0, 0, 0, 0, 0, 0x0123);
const STACK_SIZE: usize = 2 * 1024 * 1024;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");
    ctrlc::set_handler(move || std::process::exit(0)).expect("Error setting Ctrl-C handler");
    let addresses = server::addresses::Addresses::new(IPV4, IPV6, PORT);
    let listener_thread_ipv4 = thread::Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(move || server::listener_v4(addresses.clone(), blockchain::handler))
        .unwrap();
    listener_thread_ipv4.join().unwrap();
}
