use cryptography::RSA;
use env_logger;
use std::net::UdpSocket;
use std::thread::spawn;

fn main() {
    //std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();
    let cert = RSA::new(4096);
    let socket = UdpSocket::bind("0.0.0.0:8546");
}

// TODO: Update IP list
// TODO:
