use actix::prelude::*;
use actix::Actor;
use awc::Client;
use entity::Parameter;
use futures::StreamExt;
use io::SinkWrite;

use std::{fs::File, io::Read};
use ws::{WebsocketSession, WsCommand};
mod entity;
mod ws;

#[actix::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let mut sessions: Vec<(Addr<WebsocketSession>, String, String)> = Vec::with_capacity(1);

    let mut file = File::open("/Users/walker/user.txt").expect("open file error");
    let mut f = String::new();
    file.read_to_string(&mut f).expect("read lines error");
    let lines: Vec<&str> = f.split('\n').collect();
    for line in lines {
        let (response, framed) = Client::new()
            .ws("http://172.17.1.53:20000/socket.io/?EIO=3&transport=websocket&secretKey=winning")
            .connect()
            .await
            .map_err(|e| {
                println!("Error: {}", e);
            })
            .unwrap();

        println!("{:?}", response);
        let (sink, stream) = framed.split();
        let addr = WebsocketSession::create(|ctx| {
            WebsocketSession::add_stream(stream, ctx);
            WebsocketSession(SinkWrite::new(sink, ctx))
        });

        let jy: Vec<&str> = line.split(" ").collect();
        let off = Parameter::new("320482109010001", &jy[1]);
        let off_b64 = base64::encode(serde_json::to_string(&off).unwrap());
        let lg = Parameter::login("320482109010001", &jy[1]);
        let lg_b64 = base64::encode(serde_json::to_string(&lg).unwrap());
        sessions.push((
            addr,
            format!("42[\"emit\",\"{}\"]", lg_b64),
            format!("42[\"emit\",\"{}\"]", off_b64),
        ));
    }
    for session in sessions {
        &session.0.send(WsCommand("2probe".to_string())).await;
        &session.0.send(WsCommand("5".to_string())).await;
        &session.0.send(WsCommand(session.1.to_string())).await;
        &session.0.send(WsCommand(session.2.to_string())).await;
    }

    tokio::signal::ctrl_c().await.unwrap();
    println!("🎩 Ctrl-C received, shutting down");
    System::current().stop();
    Ok(())
}
