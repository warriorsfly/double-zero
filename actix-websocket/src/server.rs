use std::sync::{atomic::AtomicUsize, Arc};

use actix::Actor;
use actix_web::{middleware::Logger, web, App, HttpServer};

use crate::{
    handler::{socket_route, vistors_count},
    socket::SocketSev,
};

pub async fn serv() -> std::io::Result<()> {
    let app_state = Arc::new(AtomicUsize::new(0));
    // Start socket server actor
    let server = SocketSev::new(app_state.clone()).start();

    // Create Http server with websocket support
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(app_state.clone())
            .data(server.clone())
            .route("/count/", web::get().to(vistors_count))
            // websocket
            .service(web::resource("/studio/").to(socket_route))
        // static resources
        // .service(fs::Files::new("/static/", "static/"))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
