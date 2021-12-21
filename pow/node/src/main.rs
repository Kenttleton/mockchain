use env_logger;
use openssl::rsa::Rsa;
use std;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::net::TcpListener;
use std::thread::spawn;

fn main() {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();
    let rsa = Rsa::generate(4096).unwrap();
    let private_key: Vec<u8> = rsa.private_key_to_pem().unwrap();
    let public_key: Vec<u8> = rsa.public_key_to_pem().unwrap();

    let mut options = OpenOptions::new();
    let mut private_key_file = options
        .write(true)
        .create(true)
        .truncate(true)
        .open("certs/private.pem")
        .unwrap();
    let mut public_key_file = options
        .write(true)
        .create(true)
        .truncate(true)
        .open("certs/public.pem")
        .unwrap();
    write_to_file(private_key_file, private_key);
    write_to_file(public_key_file, public_key);

    let server = TcpListener::bind("0.0.0.0:8546").unwrap();
    for stream in server.incoming() {
        spawn(move || stream);
    }
}

fn write_to_file(mut buffer: File, data: Vec<u8>) -> std::io::Result<()> {
    let mut pos = 0;
    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }
    Ok(())
}
