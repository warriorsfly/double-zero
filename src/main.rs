

mod constants;
mod routes;
mod server;

use server::serv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    serv().await?;
    Ok(())
}
