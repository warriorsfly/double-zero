use crate::{
    addr::{Redis, Websocket, WebsocketSession},
    entity::{MessageAction, PushMessage},
};
use actix::Addr;
use actix_web::{
    web::{self, Json},
    Error, HttpRequest, HttpResponse,
};
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

// pub async fn push_msg_route(
//     msg: Json<PushMessage>,
//     redis_addr: web::Data<Addr<Redis>>,
//     srv: web::Data<Addr<Websocket>>,
// ) -> Result<MessageAction, Error> {
//     let ws_addr = srv.as_ref();
//     ws_addr.send(msg)
// }
