use std::collections::HashMap;

use actix::prelude::*;
use redis::{
    streams::{StreamMaxlen, StreamReadOptions, StreamReadReply},
    Client, Commands, RedisResult,
};

use super::message::Online;

/// 默认`通知` 通道
pub const CHANNEL_MESSAGES: &str = "channel-messages";
/// Android `通知` 通道
pub const CHANNEL_MESSAGES_ANDROID: &str = "channel-messages-android";
/// Ios `通知` 通道
pub const CHANNEL_MESSAGES_IOS: &str = "channel-messages-ios";
/// 多stream监听需要使用 block
const BLOCK_MILLIS: usize = 5000;
/// Redis Stream监听的`channels`
pub const CHANNELS: &[&str] = &[
    CHANNEL_MESSAGES,
    CHANNEL_MESSAGES_ANDROID,
    CHANNEL_MESSAGES_IOS,
];
/// 最大允许消息100000条,按照一个地区1000个医生算,每个人可以存储100条消息
const MAXLEN: StreamMaxlen = StreamMaxlen::Approx(100000);

#[derive(Message)]
#[rtype(result = "()")]
pub struct RdsMessage(pub String);
pub struct Redis {
    /// `redis client`
    rds: Client,
    sessions: HashMap<usize, Recipient<RdsMessage>>,
    /// 在线用户(创建group的)
    onlines: Vec<String>,
}

impl Actor for Redis {
    type Context = Context<Self>;
}

impl Redis {
    pub fn new(client: Client) -> Self {
        Self {
            rds: client,
            sessions: HashMap::with_capacity(1),
            onlines: vec![],
        }
    }
}

impl Handler<Online> for Redis {
    type Result = RedisResult<()>;

    fn handle(&mut self, msg: Online, _: &mut Self::Context) -> Self::Result {
        let mut con = self.rds.get_connection()?;
        // 插入onlines 设备信息,websocket session id
        con.hset("onlines", &msg.name, msg.id)?;
        // todo:cache all users with `hash`(especially redis stream group last_delivered_id,for ack)
        // 创建xgroup
        for key in CHANNELS {
            let created: RedisResult<()> = con.xgroup_create_mkstream(*key, &msg.name, "$");
            if let Err(e) = created {
                println!("Group already exists: {:?}", e);
            }
        }

        // 读取msg.name的未读消息,并推送
        let consumer = format!("channel-consumer-{}", &msg.name);
        let opts = StreamReadOptions::default()
            .block(BLOCK_MILLIS)
            .count(3)
            .group(&msg.name, consumer);

        let srr: StreamReadReply = con
            .xread_options(CHANNELS, &[">", ">", ">"], opts)
            .expect("xread group failed");

        // Ok(srr)
        Ok(())
    }
}
