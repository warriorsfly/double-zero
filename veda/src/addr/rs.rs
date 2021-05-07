use std::{collections::HashMap, usize};

use actix::{prelude::*, Recipient};

use log::info;
use redis::streams::{StreamId, StreamInfoStreamReply, StreamReadOptions};
use redis::{
    streams::{StreamKey, StreamReadReply},
    Client, Commands, Connection, RedisResult,
};

use super::WsMessage;

use crate::{
    constants::{BLOCK_MILLIS, MESSAGE_INTERVAL},
    entity::{Event, Platform},
};

/// 用户上线消息,由websocket session发送到redis
/// redis 接收到online
#[derive(Message)]
#[rtype(result = "()")]
pub struct Online {
    /// websocket session id
    pub id: usize,
    /// logined username
    pub name: String,
    /// device
    pub platform: Platform,
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
        info!("start creating redis connection for `{}`", &msg.name);
        let key_platforms = format!(r#"platforms:{}"#, &msg.name);
        let key_online_user = "online-users";
        let mut con = self
            .cli
            .get_connection()
            .expect("get redis connection error");

        let _: RedisResult<String> = con.hset(key_online_user, msg.id, msg.name.clone());
        let _: RedisResult<Platform> = con.hset(key_platforms.as_str(), msg.id, msg.platform);

        let addr = RedisSession::new(msg.id, msg.name, con, msg.addr).start();

        self.sessions.insert(msg.id, addr.recipient());
    }
}

impl Handler<Offline> for Redis {
    type Result = ();

    fn handle(&mut self, msg: Offline, _: &mut Self::Context) -> Self::Result {
        info!("name:{} disconnected, offline redis session", &msg.id);
        if let Some(session_addr) = self.sessions.get(&msg.id) {
            let key_online_user = "online-users:{}";
            let _ = session_addr.do_send(RedisOffline);
            self.sessions.remove(&msg.id);

            let mut con = self
                .cli
                .get_connection()
                .expect("get redis connection error");

            let username: RedisResult<String> = con.hget(key_online_user, msg.id);
            if let Ok(username) = username {
                let _: RedisResult<String> = con.hdel(key_online_user, msg.id);
                let key_platforms = format!(r#"platforms:{}"#, &username);
                let _: RedisResult<Platform> = con.hdel(key_platforms.as_str(), msg.id);
            }

            let _: RedisResult<Platform> = con.hget(key_online_user, msg.id);
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct RedisOffline;
pub struct RedisSession {
    pub id: usize,
    pub name: String,
    stream_name: String,
    pub session_addr: Connection,
    pub websocket_addr: Recipient<WsMessage>,
}

impl Actor for RedisSession {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        ctx.run_interval(MESSAGE_INTERVAL, |act, ctx| {
            act.read_messages(ctx);
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
            stream_name: format!("stream-messages:{}", &name),
            session_addr: connection,
            websocket_addr,
        }
    }
}

impl RedisSession {
    fn read_messages(&mut self, ctx: &mut Context<Self>) {
        let inf: RedisResult<StreamInfoStreamReply> =
            self.session_addr.xinfo_stream(&self.stream_name);
        // if inf is Err(_), the xadd command have not been execute, no message
        if let Ok(inf) = inf {
            // no message in stream,keep pollings
            if inf.length == 0 {
                return;
            }
            let opts = StreamReadOptions::default().block(BLOCK_MILLIS).count(10);

            // read all messages in the stream
            let ssr: RedisResult<StreamReadReply> =
                self.session_addr
                    .xread_options(&[&self.stream_name], &["0"], opts);
            if let Ok(ssr) = ssr {
                for StreamKey { key, ids } in ssr.keys {
                    let items: Vec<Event> = ids
                        .iter()
                        .map(|t| Event {
                            subject: t.get("subject").unwrap_or_default(),
                            act: t.get("act").unwrap_or_default(),
                            object: t.get("object").unwrap_or_default(),
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
                                        // remove all the sended messages out from stream
                                        let id_strs: &Vec<&String> =
                                            &ids.iter().map(|StreamId { id, map: _ }| id).collect();
                                        let _: RedisResult<()> =
                                            act.session_addr.xdel(key, id_strs);
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
}
