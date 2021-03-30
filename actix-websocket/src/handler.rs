use actix::Addr;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use std::time::Instant;

use crate::act::{Redis, Websocket, WebsocketSession};

pub async fn socket_route(
    req: HttpRequest,
    stream: web::Payload,
    redis: web::Data<Addr<Redis>>,
    srv: web::Data<Addr<Websocket>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WebsocketSession {
            id: 0,
            hb: Instant::now(),
            redis: redis.get_ref().clone(),
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}
