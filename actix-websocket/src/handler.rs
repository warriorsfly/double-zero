use crate::store::{Redis, Websocket, WebsocketSession};
use actix::Addr;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::time::Instant;

pub async fn socket_route(
    req: HttpRequest,
    stream: web::Payload,
    redis_addr: web::Data<Addr<Redis>>,
    srv: web::Data<Addr<Websocket>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WebsocketSession {
            id: 0,
            name: None,
            hb: Instant::now(),
            redis_addr: redis_addr.get_ref().clone(),
            websocket_addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}
