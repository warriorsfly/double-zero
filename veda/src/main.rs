#[macro_use]
extern crate lazy_static;

mod addr;
pub mod activity {
    tonic::include_proto!("activity");
}

mod config;
mod constants;
mod entity;
mod handler;
mod server;
use server::serv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    serv().await
}
