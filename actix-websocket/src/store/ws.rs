use actix::prelude::*;
use actix_web_actors::ws;
use rand::{prelude::ThreadRng, Rng};

use std::{collections::HashMap, time::Instant};

use crate::constants::{CLIENT_TIMEOUT, HEARTBEAT_INTERVAL};

use super::{Offline, Online, Redis};
#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

/// 接入websocket服务
#[derive(Message, Debug)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
}

/// 断开websocket服务
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: usize,
}
/// 告诉Studio当前session的name
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct IdentitySession {
    pub id: usize,
    pub name: String,
}

/// 告诉Studio当前session的name
#[derive(Message, Debug)]
#[rtype(result = "()")]
pub struct RedisMessage {
    pub id: usize,
    pub msg: String,
}

/// 显示在线的names
pub struct ListNames;

impl actix::Message for ListNames {
    type Result = Vec<String>;
}

pub struct Websocket {
    //链接信息
    // soc_sessions.key: websocket session的id
    // soc_sessions.value: websocket 接受参数地址
    sessions: HashMap<usize, Recipient<WsMessage>>,
    // red_sessions.key: redis steam session的id
    rng: ThreadRng,
}

impl Default for Websocket {
    fn default() -> Self {
        Self {
            sessions: HashMap::with_capacity(1),
            rng: rand::thread_rng(),
        }
    }
}

impl Websocket {
    /// 发送消息到指定name的所有客户端
    fn send_message(&self, id: usize, message: &str) {
        if let Some(addr) = self.sessions.get(&id) {
            let _ = addr.do_send(WsMessage(message.to_owned()));
        }
    }
}

impl Actor for Websocket {
    type Context = Context<Self>;
}

impl Handler<Connect> for Websocket {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        let id = self.rng.gen::<usize>();
        println!("websocket connection {} connected", id);
        self.sessions.insert(id, msg.addr);
        // 新的连接会增加连接数量,不一定会引起用户数量增加
        id
    }
}

impl Handler<Disconnect> for Websocket {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.id);
        println!("name {:?} disconnected", &msg.id);
    }
}

impl Handler<RedisMessage> for Websocket {
    type Result = ();

    fn handle(&mut self, msg: RedisMessage, _: &mut Self::Context) -> Self::Result {
        self.send_message(msg.id, &msg.msg);
    }
}

pub struct WebsocketSession {
    /// session唯一ID
    pub id: usize,
    pub name: Option<String>,
    /// session内部计时器,用于定时向客户端ping
    pub hb: Instant,
    /// websocket addr
    pub redis_addr: Addr<Redis>,
    pub websocket_addr: Addr<Websocket>,
}

impl Actor for WebsocketSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on server start.
    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        self.hb(ctx);

        // register self in socket server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        let addr = ctx.address();
        self.websocket_addr
            .send(Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with socket server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify redis server
        &self.redis_addr.do_send(Offline { id: self.id });
        // notify socket server
        &self.websocket_addr.do_send(Disconnect { id: self.id });
        Running::Stop
    }
}

/// Handle messages from socket server, we simply send it to peer server
impl Handler<WsMessage> for WebsocketSession {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebsocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        println!("websocket message: {:?}-{:?}", self.id, msg);
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                let m = text.trim();
                // we check for /sss type of messages
                if m.starts_with('/') {
                    let v: Vec<&str> = m.splitn(2, ' ').collect();
                    if let "/name" = v[0] {
                        if v.len() == 2 {
                            let name = v[1].to_owned();
                            self.name = Some(name.clone());
                            self.redis_addr.do_send(Online {
                                id: self.id,
                                name: name.clone(),
                                addr: ctx.address().recipient(),
                            });
                        } else {
                            ctx.text("!!! name is required");
                        }
                    } else {
                        ctx.text(format!("!!! unknown command: {:?}", m))
                    }
                } else {
                    ctx.text(format!("!!! unknown command: {:?}", m));
                }
            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}

impl WebsocketSession {
    /// helper method that sends ping to client every second.
    /// also this method checks pongs from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("websocket client heartbeat failed, disconnecting!");

                // notify socket server,websocket session need to be disconnect
                act.websocket_addr.do_send(Disconnect { id: act.id });
                // stop server
                ctx.stop();
                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}
