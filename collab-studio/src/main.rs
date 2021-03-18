use server::serv;

mod handler;
mod message;
mod server;
mod session;

/// 选择病人gRPC
pub mod select {
    tonic::include_proto!("patient");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    serv().await
}
