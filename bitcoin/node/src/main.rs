use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use openssl::{
    rsa::Rsa,
    ssl::{SslAcceptor, SslFiletype, SslMethod},
};
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

async fn index(_req: HttpRequest) -> impl Responder {
    "Hello."
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
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

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("./certs/private.pem", SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file("./certs/public.pem")
        .unwrap();

    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind_openssl("0.0.0.0:8080", builder)?
        .run()
        .await
}

fn write_to_file(mut buffer: File, data: Vec<u8>) -> std::io::Result<()> {
    let mut pos = 0;
    while pos < data.len() {
        let bytes_written = buffer.write(&data[pos..])?;
        pos += bytes_written;
    }
    Ok(())
}
