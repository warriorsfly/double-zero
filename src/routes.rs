
use std::time::Instant;

use actix::Addr;
use actix_web::{web, HttpRequest, HttpResponse, Error};

use actix_web_actors::ws;
use double_zero_utils::{utils::get_ip, middleware::{JwtAuth}};


use crate::{ws::{system::DoubleZeroSystem, session::WsSession},handlers::{signup, login}};

async fn ws_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<DoubleZeroSystem>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WsSession {
            id: 0,
            hb: Instant::now(),
            srv: srv.get_ref().clone(),
            ip:get_ip(&req.connection_info()),
        },
        &req,
        stream,
    )
}

pub(crate) fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
        
            .service(
                web::scope("/account")
                    .route("/signup", web::post().to(signup))
                    .route("/login", web::post().to(login)),
            ).service(
                web::scope("/users")
                .wrap(JwtAuth)
                    // .route("signup", web::post().to(user::signup))
                    // .route("login", web::post().to(user::login)),
            )
            .service(
                web::scope("/rooms")
                .wrap(JwtAuth)
                    // .route(
                    //     "/{slug}/episodes/{page_index}/{page_size}",
                    //     web::get().to(book::get_book_episodes),
                    // )
                    
            ).service(
                web::scope("/todos")
                .wrap(JwtAuth)
                    // .route(
                    //     "/{slug}/episodes/{page_index}/{page_size}",
                    //     web::get().to(book::get_book_episodes),
                    // )
                    // .route("/search/{param}", web::get().to(book::search))
                    // .route("/update/{weekday}/{page_index}/{page_size}", web::get().to(book::books_of_weekday)),
            ),
    ).service(
        web::scope("/ws").wrap(JwtAuth)
            .route("", web::get().to(ws_route))
        );
}


