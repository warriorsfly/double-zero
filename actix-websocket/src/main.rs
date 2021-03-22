use server::serv;

mod cache;
mod handler;
mod message;
mod server;
mod session;

/// 选择病人gRPC
// pub mod patient {
//     tonic::include_proto!("acc");
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    serv().await
}
