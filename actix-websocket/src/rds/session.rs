use std::time::Instant;

use actix::{Actor, Addr, Context, StreamHandler};
use redis::{
    streams::{StreamReadOptions, StreamReadReply},
    Commands, RedisResult,
};

use super::CHANNELS;

/// redis stream group block reading mills
const BLOCK_MILLIS: usize = 5000;

pub struct RedisSession {
    /// session唯一ID
    pub id: usize,
    /// session内部计时器,用于定时向客户端ping
    pub hb: Instant,
    /// 当前连接用户名
    pub client_name: String,
    /// websocket addr
    pub addr: Addr<super::Redis>,
}

impl Actor for RedisSession {
    type Context = Context<Self>;
}

// impl RedisSession {
//     fn read_messages(&self, slowness: &str) -> RedisResult<StreamReadReply> {
//         let mut con = self.rds.get_connection().expect("conn");

//         let opts = StreamReadOptions::default()
//             .block(BLOCK_MILLIS)
//             .group(&self.client_name, self.consumer_name());

//         let srr: StreamReadReply = con
//             .xread_options(CHANNELS, &[">", ">", ">"], opts)
//             .expect("messages");

//         Ok(srr)
//     }

//     fn consumer_name(&self) -> String {
//         format!("channel-consumer-{}", &self.client_name)
//     }
// }

impl StreamHandler<super::RedisMessage> for RedisSession {
    fn handle(&mut self, item: super::RedisMessage, ctx: &mut Self::Context) {
        todo!()
    }
}
