use cryptography::RSA;
use ctrlc;
use std::io::Result;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket};
use std::thread::spawn;

pub const PORT: u16 = 7645;
pub static IPV4_MULTICAST: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 123);
pub static IPV4_LOCAL: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);
pub static IPV6_MULTICAST: Ipv6Addr = Ipv6Addr::new(0xFF02, 0, 0, 0, 0, 0, 0, 0x0123);
pub static IPV6_LOCAL: Ipv6Addr = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0);

fn main() {
    // std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    // env_logger::init();
    ctrlc::set_handler(move || std::process::exit(0)).expect("Error setting Ctrl-C handler");
    let cert = RSA::new(4096);
    // loop {
    //     datagram_handler(&socket);
    // }
}

fn datagram_handler(socket: &UdpSocket) {
    // 1 MB buffer per Bitcoin spec for a block size
    let mut buf = [0; 1000000];
    let (amt, src) = socket.recv_from(&mut buf).unwrap();
    let buf = &mut buf[..amt];
    let mut message = String::new();
    for x in buf.iter() {
        message.push(*x as char);
    }
    if message == "ping" {
        socket.send_to("pong".as_bytes(), &src).unwrap();
    }
    println!("Message: {}\nSource: {}\nAmount: {}", message, &src, &amt);
}
