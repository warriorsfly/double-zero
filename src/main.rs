#[macro_use]
extern crate lazy_static;

mod config;
mod constants;
mod entity;
mod handlers;
mod route;
mod server;
mod session;

use server::serv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    serv().await
}
