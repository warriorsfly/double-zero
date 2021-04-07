use std::time::Duration;

use actix::{io::SinkWrite, prelude::*};
use actix::{Actor, Context};
use actix_codec::Framed;
use awc::{
    error::WsProtocolError,
    ws::{Codec, Frame, Message},
    BoxedSocket,
};
use bytes::Bytes;
use futures::stream::SplitSink;

pub struct WebsocketSession(pub SinkWrite<Message, SplitSink<Framed<BoxedSocket, Codec>, Message>>);

impl Actor for WebsocketSession {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsCommand(pub String);

/// Handle stdin commands
impl Handler<WsCommand> for WebsocketSession {
    type Result = ();

    fn handle(&mut self, msg: WsCommand, _ctx: &mut Context<Self>) {
        println!("Client: {}", &msg.0);
        self.0.write(Message::Text(msg.0.into()));
    }
}

/// Handle server websocket messages
impl StreamHandler<Result<Frame, WsProtocolError>> for WebsocketSession {
    fn handle(&mut self, msg: Result<Frame, WsProtocolError>, _: &mut Context<Self>) {
        println!("Server msg: {:?}", &msg);
        match msg {
            Ok(Frame::Text(txt)) => {
                // println!("Server: {:?}", txt);
            }
            Ok(Frame::Ping(_)) => {
                self.0.write(Message::Pong(Bytes::copy_from_slice(b"3")));
            }
            _ => {}
        }
    }

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Connected");
        self.0.write(Message::Text("2probe".into()));
        self.hb(ctx);
    }

    fn finished(&mut self, ctx: &mut Context<Self>) {
        println!("Server disconnected");
        ctx.stop()
    }
}

impl WebsocketSession {
    fn hb(&self, ctx: &mut Context<Self>) {
        ctx.run_later(Duration::new(6, 0), |act, ctx| {
            act.0.write(Message::Ping(Bytes::from_static(b"2")));
            act.hb(ctx);
        });
    }
}

impl actix::io::WriteHandler<WsProtocolError> for WebsocketSession {}
