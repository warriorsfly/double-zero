#[macro_use]
extern crate lazy_static;

mod config;
mod constants;
mod database;
mod entity;
mod handler;
mod seravee;
mod server;
pub mod grpc {
    tonic::include_proto!("grpc");
}

fn main() {
    println!("Hello, world!");
}
