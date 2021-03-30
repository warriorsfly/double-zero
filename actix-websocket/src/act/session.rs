use actix::{
    fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner, Handler,
    Running, StreamHandler, WrapFuture,
};
use actix_web_actors::ws;
use std::time::{Duration, Instant};

use super::{
    ws::{Connect, Disconnect, Messaging, Websocket},
    Redis,
};

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(60);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(120);

pub struct WebsocketSession {
    /// session唯一ID
    pub id: usize,
    /// session内部计时器,用于定时向客户端ping
    pub hb: Instant,
    /// 当前用户链接的redis session
    pub redis: Addr<Redis>,
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
impl Handler<Messaging> for WebsocketSession {
    type Result = ();

    fn handle(&mut self, msg: Messaging, ctx: &mut Self::Context) {
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

        println!("websocket message: {:?}", msg);
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
                    match v[0] {
                        "/name" => {
                            if v.len() == 2 {
                                let client_name = v[1].to_owned();
                                // let red =
                                //     RedisSession::new(&red, &client_name, ctx.address().clone());
                                // self.client_name = Some(v[1].to_owned());
                                // self.addr.do_send(server::IdentitySession {
                                //     id: self.id,
                                //     name: v[1].to_owned(),
                                // });
                            } else {
                                ctx.text("!!! name is required");
                            }
                        }
                        _ => ctx.text(format!("!!! unknown command: {:?}", m)),
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
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket client heartbeat failed, disconnecting!");

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
