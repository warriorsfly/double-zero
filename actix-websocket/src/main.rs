use server::serv;

mod act;
mod constants;
mod entity;
mod handler;
mod server;

/// 消息gRPC
// pub mod rpc {
//     tonic::include_proto!("message");
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    serv().await
}
