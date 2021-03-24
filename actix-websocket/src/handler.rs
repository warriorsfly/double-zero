use actix::Addr;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::time::Instant;

use crate::socket::{self, SocketSession};

pub async fn socket_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<socket::ActixWebsocket>>,
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

///  Displays and affects state
pub async fn vistors_count(count: web::Data<Arc<AtomicUsize>>) -> impl Responder {
    let current_count = count.fetch_add(0, Ordering::SeqCst);
    format!("Visitors: {}", current_count)
}
