use env_logger;
use std::net::TcpListener;
use std::thread::spawn;
use cert_gen;

fn main() {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();
    cert_gen();
    let server = TcpListener::bind("0.0.0.0:8546").unwrap();
    for stream in server.incoming() {
        spawn(move || stream);
    }
}
