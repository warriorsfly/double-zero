use actix::{Actor, Addr, Context, StreamHandler, WrapFuture};
use redis::{
    streams::{StreamId, StreamKey, StreamMaxlen, StreamReadOptions, StreamReadReply},
    Client, Commands, RedisResult,
};

use crate::application::{Application, RedisMessage};

use super::CHANNELS;

/// 多stream监听需要使用 block
const BLOCK_MILLIS: usize = 5000;
pub struct RedisSession {
    /// session唯一ID
    pub id: usize,
    pub client: Client,
    /// 当前连接用户名
    pub client_name: String,
    pub(crate) group_name: String,
    pub(crate) consumer_name: String,
    pub addr: Addr<Application>,
}

impl Actor for RedisSession {
    type Context = Context<Self>;
}

impl RedisSession {
    pub fn new(client: Client, client_name: &str, addr: Addr<Application>) -> Self {
        Self {
            id: 0,
            client,
            client_name: client_name.to_owned(),
            group_name: client_name.to_owned(),
            consumer_name: format!("channel-consumer-{}", client_name),
            addr,
        }
    }
}

impl RedisSession {
    /// 读取redis stream消息
    fn read_messages(&self) {
        let mut con = self
            .client
            .get_connection()
            .expect("get redis connection error");
        // todo:cache all users with `hash`(especially redis stream group last_delivered_id,for ack)
        // 创建xgroup
        for key in CHANNELS {
            let created: RedisResult<()> = con.xgroup_create_mkstream(*key, &self.group_name, "$");
            if let Err(e) = created {
                println!("group already exists: {:?}", e);
            }
        }

        // 读取msg.name的未读消息,并推送
        let opts = StreamReadOptions::default()
            .block(BLOCK_MILLIS)
            // .count(3)
            .group(&self.group_name, &self.consumer_name);

        let read_reply: StreamReadReply = con
            .xread_options(CHANNELS, &[">", ">", ">"], opts)
            .expect("xread group failed");

        for StreamKey { key, ids } in read_reply.keys {
            for StreamId { id, map } in &ids {
                let socket_session_id: &usize = &con
                    .hget("onlines", &self.client_name)
                    .expect("get online session id error");

                let _ = self.addr.send(RedisMessage {
                    id: *socket_session_id,
                    msg: "".to_owned(),
                });
            }
            //todo:last_delivered_id控制

            // acknowledge each stream and message ID once all messages are
            // correctly processed
            let id_strs: Vec<&String> = ids.iter().map(|StreamId { id, map: _ }| id).collect();
            con.xack(key, &self.group_name, &id_strs).expect("ack")
        }

        self.read_messages();
    }
}

// impl StreamHandler<super::RedisMessage> for RedisSession {
//     fn handle(&mut self, item: super::RedisMessage, ctx: &mut Self::Context) {
//         todo!()
//     }
// }
