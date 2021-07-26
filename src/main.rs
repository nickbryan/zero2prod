#![deny(clippy::all, clippy::pedantic)]

use std::net::TcpListener;
use zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run(TcpListener::bind("127.0.0.1:8420").expect("failed to bind address"))?.await
}
