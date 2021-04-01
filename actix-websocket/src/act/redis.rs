use std::{collections::HashMap, io::SeekFrom};

use actix::{prelude::*, Recipient};

use redis::{
    aio::{Connection, MultiplexedConnection},
    AsyncCommands, Client, RedisResult,
};

use super::WebsocketMessage;

use crate::entity::Notification;

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
use super::ws::{Connect, Disconnect, Websocket, WebsocketMessage};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(60);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(120);

/// 用户上线消息,由websocket session发送到redis
/// redis 接收到online
#[derive(Message)]
#[rtype(result = "()")]
pub struct Online {
    /// websocket session id
    pub id: usize,
    /// 客户端名称
    pub client_name: String,
    /// socket session addr
    pub addr: Recipient<WebsocketMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Offline {
    /// websocket session id
    pub id: usize,
}
// #[async_trait]
// pub trait Redis {
//     async fn read_messages(&mut self, client_name: &str) -> RedisResult<String>;
// }

pub struct Redis {
    cli: Client,
    sessions: HashMap<usize, Recipient<Online>>,
}

impl Actor for Redis {
    type Context = Context<Self>;
}
impl Redis {
    pub fn new(cli: Client) -> Self {
        Self {
            cli,
            sessions: HashMap::with_capacity(1),
        }
    }
}

impl Handler<Online> for Redis {
    type Result = ();

    fn handle(&mut self, msg: Online, ctx: &mut Self::Context) -> Self::Result {
        println!(
            "start creating redis connection for client:{}",
            &msg.client_name
        );
        let con = self
            .cli
            .get_multiplexed_tokio_connection()
            .await
            .expect("get redis multiplexed connection error");

        let addr = RedisSession::new(id, &msg.client_name, con, &msg.addr.clone()).start();

        self.sessions.insert(id, addr);
    }
}

impl Handler<Offline> for Redis {
    type Result = ();

    fn handle(&mut self, msg: Offline, ctx: &mut Self::Context) -> Self::Result {
        println!("client:{} disconnected, offline redis session", &msg.id);
        if let Some(redis) = self.sessions.get(&msg.id) {
            redis.s
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RedisSessionOffline;
pub struct RedisSession {
    pub id: usize,
    pub client_name: String,
    group_name: String,
    consumer_name: String,
    pub multiplexed_connection: MultiplexedConnection,
    pub websocket_addr: Recipient<WebsocketMessage>,
}

impl RedisSession {
    pub fn new(
        id: usize,
        client_name: &str,
        con: Connection,
        wbs: Recipient<WebsocketMessage>,
    ) -> Self {
        Self {
            id,
            client_name: client_name.to_owned(),
            group_name: format!("group-{}", &client_name),
            consumer_name: format!("group-{}-consumer-{}", &client_name, &client_name),
            multiplexed_connection: con,
            websocket_addr: wbs,
        }
    }
}

impl Actor for RedisSession {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        async {
            for key in CHANNELS {
                let created: RedisResult<()> = self
                    .multiplexed_connection
                    .xgroup_create_mkstream(*key, &self.group_name, "$")
                    .await;
                if let Err(e) = created {
                    println!("group already exists: {:?}", e);
                }
            }
            loop {
                // 读取msg.name的未读消息,并推送
                let opts = StreamReadOptions::default()
                    .block(BLOCK_MILLIS)
                    .group(client_name, client_name);

                let reply: RedisResult<StreamReadReply> = self
                    .redis
                    .xread_options(CHANNELS, &[">", ">", ">"], opts)
                    .await;

                if let Ok(reply) = reply {
                    for StreamKey { key, ids } in reply.keys {
                        let items: Vec<Notification> = ids
                            .iter()
                            .map(|t| Notification {
                                id: t.get("id").unwrap_or_default(),
                                title: t.get("title").unwrap_or_default(),
                                content: t.get("content").unwrap_or_default(),
                            })
                            .collect();
                        let res = serde_json::to_string(&items);
                        if let Ok(res) = res {
                            self.websocket_addr.send(WebsocketMessage(res));
                            let id_strs: Vec<&String> =
                                ids.iter().map(|StreamId { id, map: _ }| id).collect();
                            let _ = self.redis.xack(key, client_name, &id_strs).await?;
                        }
                    }
                }
            }
        };
    }
}

impl Handler<RedisSessionOffline> for RedisSession {
    type Result = ();

    fn handle(&mut self, _: RedisSessionOffline, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop();
    }
}
