use std::{collections::HashMap, usize};

use actix::{prelude::*, Recipient};

use redis::streams::{StreamId, StreamReadOptions};
use redis::{
    streams::{StreamKey, StreamReadReply},
    Client, Commands, Connection, RedisResult,
};

use super::WsMessage;

use crate::{
    constants::{BLOCK_MILLIS, CHANNELS, MESSAGE_INTERVAL},
    entity::Notification,
};

/// 用户上线消息,由websocket session发送到redis
/// redis 接收到online
#[derive(Message)]
#[rtype(result = "()")]
pub struct Online {
    /// websocket session id
    pub id: usize,
    /// 客户端名称
    pub name: String,
    /// socket session addr
    pub addr: Recipient<WsMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Offline {
    /// websocket session id
    pub id: usize,
}
pub struct Redis {
    cli: Client,
    sessions: HashMap<usize, Recipient<RedisOffline>>,
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

    fn handle(&mut self, msg: Online, _ctx: &mut Self::Context) -> Self::Result {
        println!("start creating redis connection for client:{}", &msg.name);
        let con = self
            .cli
            .get_connection()
            .expect("get redis connection error");

        let addr = RedisSession::new(msg.id, msg.name, con, msg.addr).start();

        self.sessions.insert(msg.id, addr.recipient());
    }
}

impl Handler<Offline> for Redis {
    type Result = ();

    fn handle(&mut self, msg: Offline, _: &mut Self::Context) -> Self::Result {
        println!("name:{} disconnected, offline redis session", &msg.id);
        if let Some(session_addr) = self.sessions.get(&msg.id) {
            let _ = session_addr.do_send(RedisOffline);
            self.sessions.remove(&msg.id);
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RedisOffline;
pub struct RedisSession {
    pub id: usize,
    pub name: String,
    group_name: String,
    consumer_name: String,
    pub redis_addr: Connection,
    pub websocket_addr: Recipient<WsMessage>,
}

impl Actor for RedisSession {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        for key in CHANNELS {
            let created: RedisResult<()> =
                self.redis_addr
                    .xgroup_create_mkstream(*key, &self.group_name, "$");
            if let Err(e) = created {
                println!("group already exists: {:?}", e);
            }
        }
        ctx.run_interval(MESSAGE_INTERVAL, |act, ctx| {
            act.consume_group_messages(ctx);
        });
    }
}

impl Handler<RedisOffline> for RedisSession {
    type Result = ();

    fn handle(&mut self, _: RedisOffline, ctx: &mut Self::Context) -> Self::Result {
        ctx.stop();
    }
}

impl RedisSession {
    pub fn new(
        id: usize,
        name: String,
        connection: Connection,
        websocket_addr: Recipient<WsMessage>,
    ) -> Self {
        Self {
            id,
            name: name.clone(),
            group_name: format!("group-{}", &name),
            consumer_name: format!("group-{}-consumer-{}", &name, &name),
            redis_addr: connection,
            websocket_addr,
        }
    }
}

impl RedisSession {
    fn consume_group_messages(&mut self, ctx: &mut Context<Self>) {
        // 读取msg.name的未读消息,并推送
        println!("group {} xgroup reading redis stream message", &self.name);
        let opts = StreamReadOptions::default()
            .block(BLOCK_MILLIS)
            .group(&self.group_name, &self.consumer_name);

        let reply: RedisResult<StreamReadReply> =
            self.redis_addr
                .xread_options(CHANNELS, &[">", ">", ">"], opts);

        if let Ok(reply) = reply {
            for StreamKey { key, ids } in reply.keys {
                if ids.is_empty() {
                    continue;
                }
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
                    self.websocket_addr
                        .send(WsMessage(res))
                        .into_actor(self)
                        .then(move |res, act, ctx| {
                            match res {
                                Ok(_) => {
                                    let id_strs: &Vec<&String> =
                                        &ids.iter().map(|StreamId { id, map: _ }| id).collect();
                                    let _: RedisResult<()> =
                                        act.redis_addr.xack(key, &act.group_name, id_strs);
                                }
                                // something wrong with socket server
                                _ => ctx.stop(),
                            }
                            fut::ready(())
                        })
                        .wait(ctx);
                }
            }
        }
    }
}
