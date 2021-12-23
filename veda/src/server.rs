use std::net::SocketAddr;

use actix::Actor;

use actix_web::{
    middleware::Logger,
    web::{self, Data},
    App, HttpServer,
};
use tonic::transport::Server;

use crate::{
    activity::activity_source_server::ActivitySourceServer,
    addr::{add_websocket, init_redis, Bridge},
    config::CONFIG,
    handler::socket_route,
};

pub async fn serv() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", &CONFIG.log);
    env_logger::init();
    let redis_addr = init_redis(&CONFIG.redis_url);
    let addr: SocketAddr = CONFIG.grpc_url.parse().unwrap();

    let seravee = Bridge {
        ws_addr: addr,
        redis_addr: redis_addr.clone().recipient(),
    };

    let seravee_addr = seravee.clone().start();
    actix_web::rt::spawn(async move {
        let _ = Server::builder()
            .add_service(ActivitySourceServer::new(seravee))
            .serve(addr)
            .await;
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(add_websocket)
            .app_data(Data::new(redis_addr.clone()))
            .app_data(Data::new(seravee_addr.clone()))
            .service(web::resource("/ws/").to(socket_route))
    })
    .bind(&CONFIG.server)?
    .run()
    .await
}
