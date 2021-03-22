use actix::Addr;
use actix_web::{web, Error, HttpRequest, HttpResponse, Responder};
use actix_web_actors::ws;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::time::Instant;

use crate::{server, session::WinSession};

pub async fn studio_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<server::WinWebsocket>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WinSession {
            id: 0,
            hb: Instant::now(),
            identity: None,
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
