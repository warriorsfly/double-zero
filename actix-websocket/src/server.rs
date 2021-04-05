use actix_web::{middleware::Logger, web, App, HttpServer};

use crate::{
    config::CONFIG,
    handler::socket_route,
    store::{add_redis, add_websocket},
};

pub async fn serv() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", &CONFIG.log);
    env_logger::init();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(add_websocket)
            .configure(add_redis)
            .service(web::resource("/ws/").to(socket_route))
    })
    .bind(&CONFIG.server)?
    .run()
    .await
}
