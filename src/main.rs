use server::serv;

mod handler;
mod message;
mod server;
mod session;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    serv().await
}
