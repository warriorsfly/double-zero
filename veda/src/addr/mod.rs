mod rs;
mod seravee;
mod ws;

use crate::config::CONFIG;
use actix::{Actor, Addr};
use actix_web::web;
use redis::Client;

use self::seravee::Seravee;
pub(crate) use self::{rs::*, ws::*};

pub struct Exia {}

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
