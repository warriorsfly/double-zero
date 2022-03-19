
use std::{collections::{HashMap, HashSet}, time::{Instant, Duration}};

use actix::{Addr, Actor, Handler, StreamHandler, AsyncContext, WrapFuture, ActorFutureExt, ContextFutureSpawner, ActorContext};
use actix_web::{web::{self, ServiceConfig, Data}, HttpRequest, HttpResponse, Error};
use actix_web_actors::ws;
use double_zero_utils::{pool::init_pool, utils::get_ip, ConnectionId, IpAddr, RoomId};
use tracing::debug;

use crate::{system::DoubleZeroSystem, messages::{Connect, Disconnect, WsMessage}};

struct WsSession {
    id: ConnectionId,
    hb: Instant,
    srv: Addr<DoubleZeroSystem>,
    pub ip: IpAddr,
    pub rooms: Option<HashSet<RoomId>>,
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
        self
        .srv
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
                // let m = text.trim();
                // // we check for /sss type of messages
                // if m.starts_with('/') {
                //     let v: Vec<&str> = m.splitn(2, ' ').collect();
                //     match v[0] {
                //         // "/list" => {
                //         //     // Send ListRooms message to chat server and wait for
                //         //     // response
                //         //     println!("List rooms");
                //         //     self.srv
                //         //         .send(server::ListRooms)
                //         //         .into_actor(self)
                //         //         .then(|res, _, ctx| {
                //         //             match res {
                //         //                 Ok(rooms) => {
                //         //                     for room in rooms {
                //         //                         ctx.text(room);
                //         //                     }
                //         //                 }
                //         //                 _ => println!("Something is wrong"),
                //         //             }
                //         //             fut::ready(())
                //         //         })
                //         //         .wait(ctx)
                //         //     // .wait(ctx) pauses all events in context,
                //         //     // so actor wont receive any new messages until it get list
                //         //     // of rooms back
                //         // }
                //         // "/join" => {
                //             if v.len() == 2 {
                //                 // self.room = v[1].to_owned();
                //                 self.srv.do_send(JoinRoom {
                //                     websocket_id: self.id,
                //                     room_id: self.v[1].to_owned(),
                //                 });

                //                 ctx.text("joined");
                //             } else {
                //                 ctx.text("!!! room name is required");
                //             }
                //         }
                //         "/name" => {
                //             if v.len() == 2 {
                //                 self.name = Some(v[1].to_owned());
                //             } else {
                //                 ctx.text("!!! name is required");
                //             }
                //         }
                //         _ => ctx.text(format!("!!! unknown command: {:?}", m)),
                //     }
                // } else {
                //     let msg = if let Some(ref name) = self.name {
                //         format!("{}: {}", name, m)
                //     } else {
                //         m.to_owned()
                //     };
                //     // send message to chat server
                //     self.srv.do_send(server::ClientMessage {
                //         id: self.id,
                //         msg,
                //         room: self.room.clone(),
                //     })
                // }
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

pub fn config_double_zero_system(cfg:&mut ServiceConfig){
    let tas =  DoubleZeroSystem{
        pool:init_pool(),
        sessions: HashMap::new(),
        rooms: HashMap::new(),
        rng: rand::thread_rng(),
    };
    cfg.app_data(Data::new(tas));
}

pub async fn config_ws_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<DoubleZeroSystem>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WsSession {
            id: 0,
            hb: Instant::now(),
            srv: srv.get_ref().clone(),
            ip:get_ip(&req.connection_info()),
            rooms: None,
        },
        &req,
        stream,
    )
}


pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/users")
                    // .route("signup", web::post().to(user::signup))
                    // .route("login", web::post().to(user::login)),
            )
            .service(
                web::scope("/rooms")
                    // .route(
                    //     "/{slug}/episodes/{page_index}/{page_size}",
                    //     web::get().to(book::get_book_episodes),
                    // )
                    
            ).service(
                web::scope("/todos")
                    // .route(
                    //     "/{slug}/episodes/{page_index}/{page_size}",
                    //     web::get().to(book::get_book_episodes),
                    // )
                    // .route("/search/{param}", web::get().to(book::search))
                    // .route("/update/{weekday}/{page_index}/{page_size}", web::get().to(book::books_of_weekday)),
            ),
    );
    //.service(route("/ws", web::get().to(config_ws_route)));
}
