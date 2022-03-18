
pub mod handlers;
mod messages;
mod session;
mod server;

// pub ROOM_NAME:&str = "room";

use std::collections::HashMap;

use actix::AsyncContext;
use actix_web::web::ServiceConfig;
use server::TransAmSystem;
use session::Session;

pub fn config_trans_am_system(cfg:&mut ServiceConfig){
    let tas =  TransAmSystem{
        sessions: HashMap::new(),
        rooms: HashMap::new(),
        rng: rand::thread_rng(),
    };
    cfg.app_data(Data::new(tas));
}

pub async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<TransAmSystem>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        Session {
            id: 0,
            hb: Instant::now(),
            addr: srv.get_ref().clone(),
            ip:&req.address(),
            rooms: HashMap::new(),
        },
        &req,
        stream,
    )
}
