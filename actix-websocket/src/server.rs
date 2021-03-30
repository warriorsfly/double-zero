use actix::Actor;
use actix_web::{middleware::Logger, web, App, HttpServer};

use crate::{application::Application, handler::socket_route};

pub async fn serv() -> std::io::Result<()> {
    // Start socket server actor
    let server = Application::default().start();

    // Create Http server with websocket support
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(server.clone())
            .service(web::resource("/notify/").to(socket_route))
        // static resources
        // .service(fs::Files::new("/static/", "static/"))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
