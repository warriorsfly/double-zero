use server::serv;

mod handler;
mod rds;
mod server;
mod socket;

/// 选择病人gRPC
// pub mod patient {
//     tonic::include_proto!("acc");
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    serv().await
}
