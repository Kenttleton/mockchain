use cryptography::RSA;
use ctrlc;
use std::io::Result;
use std::net::UdpSocket;
use std::thread::spawn;

fn main() {
    // std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    // env_logger::init();
    let cert = RSA::new(4096);
    let socket = UdpSocket::bind("127.0.0.1:8546").unwrap();
    socket.set_nonblocking(true).unwrap();
    // 1 MB buffer per Bitcoin spec for a block size
    let mut buf = [0; 1000000];
    let (amt, src) = socket.recv_from(&mut buf).unwrap();
    let buf = &mut buf[..amt];
    buf.reverse();
    println!("Source: {}\nAmount: {}", &src, &amt);
    socket.send_to(buf, &src).unwrap();
}

fn handler() {}
