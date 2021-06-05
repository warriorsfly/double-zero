use std::net::SocketAddr;

use actix::Actor;

use actix_web::{middleware::Logger, web, App, HttpServer};
use tonic::transport::Server;

use crate::{
    addr::{add_websocket, init_redis, Seravee},
    config::CONFIG,
    grpc::message_source_server::MessageSourceServer,
    handler::socket_route,
};

pub async fn serv() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", &CONFIG.log);
    env_logger::init();
    let redis_addr = init_redis(&CONFIG.redis_url);
    let addr: SocketAddr = CONFIG.grpc_url.parse().unwrap();

    let seravee = Seravee {
        addr: addr,
        redis_addr: redis_addr.clone().recipient(),
    };

    let seravee_addr = seravee.clone().start();
    actix_web::rt::spawn(async move {
        let _ = Server::builder()
            .add_service(MessageSourceServer::new(seravee))
            .serve(addr)
            .await;
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(add_websocket)
            .data(redis_addr.clone())
            .data(seravee_addr.clone())
            .service(web::resource("/vp/").to(socket_route))
    })
    .bind(&CONFIG.server)?
    .run()
    .await
}
