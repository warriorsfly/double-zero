#[macro_use]
extern crate lazy_static;

mod addr;
mod config;
mod constants;
mod entity;
mod handler;
mod server;
use server::serv;

pub mod grpc {
    tonic::include_proto!("grpc");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    serv().await
}
