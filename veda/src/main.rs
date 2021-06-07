#[macro_use]
extern crate lazy_static;

mod addr;
mod config;
mod constants;
mod entity;
mod handler;
mod server;
use server::serv;

pub mod activity {
    tonic::include_proto!("activity");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    serv().await
}
