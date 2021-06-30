mod rs;
mod seravee;
mod ws;

use actix::{Actor, Addr};
use actix_web::web::{self, Data};
use redis::Client;

pub(crate) use self::{rs::*, seravee::*, ws::*};

pub fn init_redis(redis_url: &str) -> Addr<Redis> {
    let cli = Client::open(redis_url)
        .expect(format!("unable to connect to redis:{}", redis_url).as_str());
    Redis::new(cli).start()
}

pub fn add_websocket(cfg: &mut web::ServiceConfig) {
    let addr = Websocket::default().start();
    cfg.app_data(Data::new(addr));
}
