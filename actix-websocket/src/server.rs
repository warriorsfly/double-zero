use actix_web::{middleware::Logger, web, App, HttpServer};

use crate::{
    handler::socket_route,
    store::{add_redis, add_websocket},
};

pub async fn serv() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .configure(add_websocket)
            .configure(add_redis)
            .service(web::resource("/notify/").to(socket_route))
        // static resources
        // .service(fs::Files::new("/static/", "static/"))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
