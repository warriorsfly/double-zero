use std::time::{Duration, Instant};

use actix::{
    prelude::*, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner,
    Handler, Recipient, StreamHandler, WrapFuture,
};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};

use tracing::debug;

use double_zero_utils::{middleware::JwtAuth, utils::get_ip, ConnectionId, IpAddr, RoomId};

use crate::{
    handlers::{login, signup},
    ws::system::DoubleZeroSystem,
};

pub(crate) struct WsSession {
    pub(crate) id: ConnectionId,
    pub(crate) hb: Instant,
    pub(crate) srv: Addr<DoubleZeroSystem>,
    pub(crate) ip: IpAddr,
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        self.hb(ctx);

        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // across all routes within application
        let addr = ctx.address();
        self.srv
            .send(Connect {
                addr: addr.recipient(),
                ip: self.ip.to_owned(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                actix::fut::ready(())
            })
            .wait(ctx);
    }
}

impl WsSession {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(Duration::from_secs(25), |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > Duration::from_secs(60) {
                // heartbeat timed out
                debug!("Websocket Client heartbeat failed, disconnecting!");

                // notify veda server
                act.srv.do_send(Disconnect {
                    websocket_id: act.id,
                    ip: act.ip.to_owned(),
                });

                // stop actor
                ctx.stop();

                // stop sending pings
                return;
            }

            ctx.ping(b"");
        });
    }
}

impl Handler<WsMessage> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        debug!("WEBSOCKET MESSAGE: {:?}", msg);
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
                        //     // Send ListRooms message to chat server and wait for
                        //     // response
                        //     println!("List rooms");
                        //     self.srv
                        //         .send(ListRooms)
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
                        //     // so actor wont receive any new messages until it get list
                        //     // of rooms back
                        // }
                        "/join" => {
                            if v.len() == 2 {
                                // self.room = v[1].to_owned();
                                // self.srv.do_send(JoinRoom {
                                //     websocket_id: self.id,
                                //     room_id: RoomId::parse(v[1].to_owned()),
                                // });

                                ctx.text("joined");
                            } else {
                                ctx.text("!!! room name is required");
                            }
                        }

                        _ => ctx.text(format!("!!! unknown command: {:?}", m)),
                    }
                } else {
                    // let msg = if let Some(ref name) = self.name {
                    //     format!("{}: {}", name, m)
                    // } else {
                    //     m.to_owned()
                    // };
                    // // send message to chat server
                    // self.srv.do_send(server::ClientMessage {
                    //     id: self.id,
                    //     msg,
                    //     room: self.room.clone(),
                    // })
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

/// trans am server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

/// Message for chat server communications
/// New trans am session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub ip: IpAddr,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub websocket_id: ConnectionId,
    pub ip: IpAddr,
}

/// The messages sent to websocket clients
#[derive(Serialize, Deserialize, Message)]
#[rtype(result = "Result<String, std::convert::Infallible>")]
pub struct TextMessage {
    /// Id of the client session
    pub websocket_id: ConnectionId,
    /// Peer message
    pub msg: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinRoom {
    pub room_id: RoomId,
    pub websocket_id: ConnectionId,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct LeaveRoom {
    pub room_id: RoomId,
    pub websocket_id: ConnectionId,
}
