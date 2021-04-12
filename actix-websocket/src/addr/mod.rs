mod cs;
mod grpc;
mod ws;

use crate::{config::CONFIG, message::actix_message_client::ActixMessageClient};
use actix::{Actor, Addr};
use actix_web::web;
use redis::Client;

pub(crate) use self::{cs::*, grpc::*, ws::*};

fn init_redis(redis_url: &str) -> Addr<Redis> {
    let cli = Client::open(redis_url)
        .expect(format!("unable to connect to redis:{}", redis_url).as_str());
    Redis::new(cli).start()
}

pub fn add_redis(cfg: &mut web::ServiceConfig) {
    let addr = init_redis(&CONFIG.redis_url);
    cfg.data(addr);
}

pub fn add_websocket(cfg: &mut web::ServiceConfig) {
    let addr = Websocket::default().start();
    cfg.data(addr);
}

pub async fn init_rpc(rpc_url: &'static str) -> MsgRpc {
    let cli = ActixMessageClient::connect(rpc_url)
        .await
        .expect(format!("unable to connect to redis:{}", rpc_url).as_str());
    MsgRpc { cli }
}
