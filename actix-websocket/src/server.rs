use actix::Actor;
use actix_web::{middleware::Logger, web, App, HttpServer};

use crate::{
    addr::{add_redis, add_websocket, init_rpc},
    config::CONFIG,
    handler::socket_route,
};

pub async fn serv() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", &CONFIG.log);
    env_logger::init();
    // let rpc_addr = init_rpc(&CONFIG.grpc_url).await.start();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(add_websocket)
            .configure(add_redis)
            // .data(rpc_addr.clone())
            .service(web::resource("/ws/").to(socket_route))
    })
    .bind(&CONFIG.server)?
    .run()
    .await
}
