use crate::act::{Websocket, WebsocketSession};
use actix::Addr;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use redis::Client;
use std::time::Instant;

pub async fn socket_route(
    req: HttpRequest,
    stream: web::Payload,
    cli: web::Data<Client>,
    srv: web::Data<Addr<Websocket>>,
) -> Result<HttpResponse, Error> {
    let redis = cli
        .get_ref()
        .get_async_connection()
        .await
        .expect("get redis connection error");
    ws::start(
        WebsocketSession {
            id: 0,
            hb: Instant::now(),
            redis,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}
