use actix::Addr;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::time::Instant;

use crate::{
    application::Application,
    socket::{self, SocketSession},
};

pub async fn socket_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<Application>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        SocketSession {
            id: 0,
            hb: Instant::now(),
            client_name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}
