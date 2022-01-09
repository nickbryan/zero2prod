#![deny(clippy::all, clippy::pedantic)]

use std::net::TcpListener;
use zero2prod::configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = configuration::get().expect("failed to read configuration");

    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listener = TcpListener::bind(address)?;

    zero2prod::run(listener)?.await
}
