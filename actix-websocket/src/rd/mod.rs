use actix::{prelude::*, Actor, Context, Recipient};
use redis::{streams::StreamMaxlen, Client, Connection, RedisResult};

mod message;
mod session;
mod sr;

pub(crate) use {message::*, session::*};

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

#[derive(Message)]
#[rtype(result = "RedisResult<Connection>")]
pub struct RedisConnectionRequest;

pub struct Redis {
    cli: Client,
    sessions: Vec<Recipient<Online>>,
}

impl Actor for Redis {
    type Context = Context<Self>;
}

impl Handler<RedisConnectionRequest> for Redis {
    type Result = RedisResult<Connection>;

    fn handle(&mut self, _: RedisConnectionRequest, _: &mut Self::Context) -> Self::Result {
        self.cli.get_connection()
    }
}
