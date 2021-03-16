use server::serv;

mod handler;
mod message;
mod server;
mod session;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tonic_build::configure()
        .build_server(true)
        .compile(&["proto/patient.proto"], &["proto/patient"])?;
    serv().await
}
