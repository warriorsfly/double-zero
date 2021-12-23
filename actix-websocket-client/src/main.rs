//! Simple websocket client.
use std::time::Duration;
use std::{io, thread};

use actix::io::SinkWrite;
use actix::*;
use actix_codec::Framed;
use awc::{
    error::WsProtocolError,
    ws::{Codec, Frame, Message},
    BoxedSocket, Client,
};
use bytes::Bytes;
use futures::stream::{SplitSink, StreamExt};

#[actix::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let (response, framed) = Client::new()
        .ws("http://127.0.0.1:8080")
        .connect()
        .await
        .map_err(|e| {
            println!("Error: {}", e);
        })
        .unwrap();

    println!("{:?}", response);
    let (sink, stream) = framed.split();
    let addr = ChatClient::create(|ctx| {
        ChatClient::add_stream(stream, ctx);
        ChatClient(SinkWrite::new(sink, ctx))
    });

    // start console loop
    thread::spawn(move || loop {
        let mut cmd = String::new();
        if io::stdin().read_line(&mut cmd).is_err() {
            println!("error");
            return;
        }
        addr.do_send(WebsocketCommand(cmd));
    });
    tokio::signal::ctrl_c().await.unwrap();
    println!("🎩 Ctrl-C received, shutting down");
    System::current().stop();
    Ok(())
}

struct ChatClient(SinkWrite<Message, SplitSink<Framed<BoxedSocket, Codec>, Message>>);

#[derive(Message)]
#[rtype(result = "()")]
struct WebsocketCommand(String);

impl Actor for ChatClient {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        // start heartbeats otherwise server will disconnect after 10 seconds
        self.hb(ctx)
    }

    fn stopped(&mut self, _: &mut Context<Self>) {
        println!("Disconnected");

        // Stop application on disconnect
        System::current().stop();
    }
}

impl ChatClient {
    fn hb(&self, ctx: &mut Context<Self>) {
        ctx.run_later(Duration::new(120, 0), |act, ctx| {
            let _ = act.0.write(Message::Ping(Bytes::from_static(b"")));
            act.hb(ctx);

            // client should also check for a timeout here, similar to the
            // server code
        });
    }
}

/// Handle stdin commands
impl Handler<WebsocketCommand> for ChatClient {
    type Result = ();

    fn handle(&mut self, msg: WebsocketCommand, _ctx: &mut Context<Self>) {
        let _ = self.0.write(Message::Text(msg.0.into()));
    }
}

/// Handle server websocket messages
impl StreamHandler<Result<Frame, WsProtocolError>> for ChatClient {
    fn handle(&mut self, msg: Result<Frame, WsProtocolError>, _: &mut Context<Self>) {
        // if let Ok(Frame::Text(txt)) = msg {
        //     println!("Server: {:?}", txt)
        // }

        // if let Ok(Frame::Ping(_)) = msg {
        //     self.0.write(Message::Pong(Bytes::from_static(b"")));
        // }

        match msg {
            Ok(Frame::Text(txt)) => {
                println!("Server: {:?}", txt);
            }
            Ok(Frame::Ping(_)) => {
                let _ = self.0.write(Message::Pong(Bytes::from_static(b"")));
            }
            _ => {}
        }
    }

    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("Connected");
    }

    fn finished(&mut self, ctx: &mut Context<Self>) {
        println!("Server disconnected");
        ctx.stop()
    }
}

impl actix::io::WriteHandler<WsProtocolError> for ChatClient {}
