#[macro_use]
extern crate lazy_static;

mod config;
mod constants;
mod entity;
mod handler;
mod server;

mod mq;
mod websocket;
use server::serv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    serv().await
}
