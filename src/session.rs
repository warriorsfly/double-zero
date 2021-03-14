use std::time::{Duration, Instant};

use actix::*;

use actix_web_actors::ws;

use crate::server;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct StudioSession {
    /// session唯一ID
    pub id: usize,
    /// session内部计时器,用于定时向客户端ping
    pub hb: Instant,
    /// 当前连接用户名
    pub name: Option<String>,
    /// websocket addr
    pub addr: Addr<server::StudioWebsocket>,
}

impl Actor for StudioSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on server start.
    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        self.hb(ctx);

        // register self in planet server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        let addr = ctx.address();
        self.addr
            .send(server::Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with planet server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify planet server
        self.addr.do_send(server::Disconnect { id: self.id });
        Running::Stop
    }
}

/// Handle messages from planet server, we simply send it to peer server
impl Handler<server::Message> for StudioSession {
    type Result = ();

    fn handle(&mut self, msg: server::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for StudioSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        println!("WEBSOCKET MESSAGE: {:?}", msg);
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
                    match v[0] {
                        // "/list" => {
                        //     // Send ListRooms message to planet server and wait for
                        //     // response
                        //     println!("List rooms");
                        //     self.addr
                        //         .send(server::ListRooms)
                        //         .into_actor(self)
                        //         .then(|res, _, ctx| {
                        //             match res {
                        //                 Ok(rooms) => {
                        //                     for room in rooms {
                        //                         ctx.text(room);
                        //                     }
                        //                 }
                        //                 _ => println!("Something is wrong"),
                        //             }
                        //             fut::ready(())
                        //         })
                        //         .wait(ctx)
                        //     // .wait(ctx) pauses all events in context,
                        //     // so server wont receive any new messages until it get list
                        //     // of rooms back
                        // }
                        // "/join" => {
                        //     if v.len() == 2 {
                        //         self.room = v[1].to_owned();
                        //         self.addr.do_send(server::Join {
                        //             id: self.id,
                        //             name: self.room.clone(),
                        //         });

                        //         ctx.text("joined");
                        //     } else {
                        //         ctx.text("!!! room name is required");
                        //     }
                        // }
                        "/name" => {
                            if v.len() == 2 {
                                self.addr.do_send(server::NameSession {
                                    id: self.id,
                                    name: v[1].to_owned(),
                                });
                                // let name = v[1].to_owned();
                                // self.addr.self.name = Some(v[1].to_owned());
                            } else {
                                ctx.text("!!! name is required");
                            }
                        }
                        _ => ctx.text(format!("!!! unknown command: {:?}", m)),
                    }
                } else {
                    // ctx.text(format!("!!! unknown command: {:?}", m));
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

impl StudioSession {
    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket client heartbeat failed, disconnecting!");

                // notify planet server
                act.addr.do_send(server::Disconnect { id: act.id });

                // stop server
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}
