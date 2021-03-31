use actix::{prelude::*, Actor, Context, Recipient};
use redis::{
    streams::{StreamId, StreamKey, StreamMaxlen, StreamReadOptions, StreamReadReply},
    Client, Commands, Connection, RedisResult,
};

/// 默认`通知` 通道
const CHANNEL_MESSAGES: &str = "channel-messages";
/// Android `通知` 通道
const CHANNEL_MESSAGES_ANDROID: &str = "channel-messages-android";
/// Ios `通知` 通道
const CHANNEL_MESSAGES_IOS: &str = "channel-messages-ios";

/// Redis Stream监听的`channels`
const CHANNELS: &[&str] = &[
    CHANNEL_MESSAGES,
    CHANNEL_MESSAGES_ANDROID,
    CHANNEL_MESSAGES_IOS,
];

/// 最大允许消息100000条,按照一个地区1000个医生算,每个人可以存储100条消息
const MAXLEN: StreamMaxlen = StreamMaxlen::Approx(100000);

/// 多stream监听需要使用 block
const BLOCK_MILLIS: usize = 5000;
pub struct Redis {
    cli: Client,
    handles: Vec<String>,
}

impl Actor for Redis {
    type Context = Context<Self>;
}

impl Redis {
    /// 读取redis stream消息
    fn read_messages(&self, client_name: &str) {
        let mut con = self
            .cli
            .get_connection()
            .expect("get redis connection error");
        // todo:cache all users with `hash`(especially redis stream group last_delivered_id,for ack)
        // 创建xgroup
        for key in CHANNELS {
            let created: RedisResult<()> = con.xgroup_create_mkstream(*key, client_name, "$");
            if let Err(e) = created {
                println!("group already exists: {:?}", e);
            }
        }

        // 读取msg.name的未读消息,并推送
        let opts = StreamReadOptions::default()
            .block(BLOCK_MILLIS)
            // .count(3)
            .group(client_name, client_name);

        let read_reply: StreamReadReply = con
            .xread_options(CHANNELS, &[">", ">", ">"], opts)
            .expect("xread group failed");

        for StreamKey { key, ids } in read_reply.keys {
            for StreamId { id, map } in &ids {
                let socket_session_id: &usize = &con
                    .hget("onlines", client_name)
                    .expect("get online session id error");

                // let _ = self.addr.send(RedisMessage {
                //     id: *socket_session_id,
                //     msg: "".to_owned(),
                // });
            }
            //todo:last_delivered_id控制

            // acknowledge each stream and message ID once all messages are
            // correctly processed
            let id_strs: Vec<&String> = ids.iter().map(|StreamId { id, map: _ }| id).collect();
            con.xack(key, client_name, &id_strs).expect("ack")
        }

        self.read_messages(client_name);
    }
}
