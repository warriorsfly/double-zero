use server::serv;

mod act;
mod handler;
// mod rd;
mod server;

/// 选择病人gRPC
// pub mod patient {
//     tonic::include_proto!("acc");
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    serv().await
}
