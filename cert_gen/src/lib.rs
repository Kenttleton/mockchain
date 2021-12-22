use openssl::rsa::Rsa;
use std;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

const BITES: u32 = 4096;
const PRIVATE_KEY_LOCATION: &str = "certs/private.pem";
const PUBLIC_KEY_LOCATION: &str = "certs/public.pem";

pub fn create() {
    let rsa = Rsa::generate(BITES).unwrap();
    let private_key: Vec<u8> = rsa.private_key_to_pem().unwrap();
    let public_key: Vec<u8> = rsa.public_key_to_pem().unwrap();

    let mut options = OpenOptions::new();
    let private_key_file = options
        .write(true)
        .create(true)
        .truncate(true)
        .open(PRIVATE_KEY_LOCATION)
        .unwrap();
    let public_key_file = options
        .write(true)
        .create(true)
        .truncate(true)
        .open(PUBLIC_KEY_LOCATION)
        .unwrap();
    write_to_file(private_key_file, private_key);
    write_to_file(public_key_file, public_key);
}

fn write_to_file(mut buffer: File, data: Vec<u8>) {
    let mut pos = 0;
    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..]);
        match bytes_written {
            Ok(x) => pos += x,
            Err(x) => println!("{}", x),
        }
    }
    println!("Cert Generated");
}
