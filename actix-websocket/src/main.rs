#[macro_use]
extern crate lazy_static;

mod addr;
mod config;
mod constants;
mod entity;
mod handler;
mod server;

use server::serv;

/// message gRPC client
pub mod message {
    tonic::include_proto!("message");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    serv().await
}
