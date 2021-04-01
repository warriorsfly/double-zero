use actix::prelude::*;
use rand::{prelude::ThreadRng, Rng};

use std::collections::HashMap;
#[derive(Message)]
#[rtype(result = "()")]
pub struct WebsocketMessage(pub String);

/// 接入websocket服务
#[derive(Message, Debug)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<WebsocketMessage>,
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
    sessions: HashMap<usize, Recipient<WebsocketMessage>>,
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
            let _ = addr.do_send(WebsocketMessage(message.to_owned()));
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
        println!("client {:?} disconnected", &msg.id);
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
    /// 当前用户链接的redis session
    pub redis: Connection,
    /// websocket addr
    pub addr: Addr<Websocket>,
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
        self.addr
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
        // notify socket server
        self.addr.do_send(Disconnect { id: self.id });
        Running::Stop
    }
}

/// Handle messages from socket server, we simply send it to peer server
impl Handler<WebsocketMessage> for WebsocketSession {
    type Result = ();

    fn handle(&mut self, msg: WebsocketMessage, ctx: &mut Self::Context) {
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
                //如果socket连接没有name,暂时不处理传输数据
                //todo 添加错误返回信息
                // if self.identity == None {
                //     return;
                // }
                let m = text.trim();
                // we check for /sss type of messages
                if m.starts_with('/') {
                    let v: Vec<&str> = m.splitn(2, ' ').collect();
                    if let "/name" = v[0] {
                        if v.len() == 2 {
                            let client_name = v[1].to_owned();
                            self.name = client_name.clone();
                            // ctx.run_interval(Duration::from_millis(200), async {
                            //     self.read_messages(&client_name).await
                            // });
                            // let execution = async {
                            //     loop {
                            //         let str =
                            //             self.read_messages(&client_name).await.unwrap_or_default();
                            //         if str.is_empty() {
                            //             return;
                            //         }
                            //         ctx.text(str);
                            //     }
                            // };
                            // let arb = Arbiter::new();
                            // arb.spawn(execution);
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
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if &self.name.is_none() {
                // heartbeat timed out
                println!("websocket name is none for a long time, disconnecting...");

                // notify socket server
                act.addr.do_send(Disconnect { id: act.id });

                // stop server
                ctx.stop();

                // don't try to send a ping
                return;
            }
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("websocket client heartbeat failed, disconnecting!");

                // notify socket server
                act.addr.do_send(Disconnect { id: act.id });

                // stop server
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}
