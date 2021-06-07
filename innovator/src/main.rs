#[macro_use]
extern crate lazy_static;

mod config;
mod constants;
mod database;
mod entity;
mod handler;
mod seravee;
mod server;
pub mod activity {
    tonic::include_proto!("activity");
}

fn main() {
    println!("Hello, world!");
}
