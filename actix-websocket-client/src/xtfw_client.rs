//! Simple websocket client.
use std::{fs::File, io, sync::Arc, thread};
use std::{io::Read, time::Duration};

use actix::io::SinkWrite;
use actix::*;
use actix_codec::Framed;
use awc::{
    error::WsProtocolError,
    ws::{Codec, Frame, Message},
    BoxedSocket, Client,
};
// use base64;
use bytes::Bytes;
use futures::stream::{SplitSink, StreamExt};

use serde::{self, Deserialize, Serialize};
//{"YLJGDM":"320482109010001","KSDM":"01","YSGH":"7803","JZHZZJHM":"320100192806289851","JZHZZJLX":"01","IP":"","MAC":""
//,"METHOD":"offilneMsg","MSGLX":"1"}
#[derive(Deserialize, Serialize)]
pub struct Parameter {
    /// 机构编码
    #[serde(rename = "YLJGDM")]
    pub jgdm: String,

    /// 科室代码
    #[serde(rename = "KSDM")]
    pub ksdm: String,

    /// 医生工号
    #[serde(rename = "YSGH")]
    pub ysgh: String,

    /// 证件号码
    #[serde(rename = "JZHZZJHM")]
    pub zjhm: String,

    /// 证件类型
    #[serde(rename = "JZHZZJLX")]
    pub zjlx: String,

    /// ip
    #[serde(rename = "IP")]
    pub ip: String,

    /// mac
    #[serde(rename = "MAC")]
    pub mac: String,

    /// method
    #[serde(rename = "METHOD")]
    pub method: String,

    /// msglx
    #[serde(rename = "MSGLX")]
    pub msglx: String,
}

impl Parameter {
    pub fn new(jgdm: &str, ysgh: &str) -> Self {
        Self {
            jgdm: jgdm.to_owned(),
            ksdm: "01".to_string(),
            ysgh: ysgh.to_owned(),
            zjhm: "320100192806289851".to_owned(),
            zjlx: "01".to_owned(),
            ip: "".to_owned(),
            mac: "".to_owned(),
            method: "offilneMsg".to_owned(),
            msglx: "1".to_owned(),
        }
    }
}

fn main() {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let sys = System::new("websocket-client");

    let mut file = File::open("user.txt").expect("open file error");
    let mut f = String::new();
    file.read_to_string(&mut f).expect("read lines error");
    let lines: Vec<&str> = f.split(' ').collect();

    // let lines = Arc::new(lines);

    for line in lines {
        Arbiter::spawn(move || async {
            let (response, framed) = Client::new()
                .ws("http://127.0.0.1:20000/socket.io/?EIO=3&transport=websocket&secretKey=winning")
                // .ws("http://127.0.0.1:3000/notify/")
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
                addr.do_send(ClientCommand(cmd));
            });

            let jy: Vec<&str> = line.split(" ").collect();
            let p = Parameter::new(&jy[0], &jy[1]);

            let b64 = base64::encode(serde_json::to_string(&p).unwrap());
            let p = format!("42[\"emit\":\"{}\"]", b64);
            addr.do_send(ClientCommand(p));
        });
    }
    sys.run().unwrap();
}

struct ChatClient(SinkWrite<Message, SplitSink<Framed<BoxedSocket, Codec>, Message>>);

#[derive(Message)]
#[rtype(result = "()")]
struct ClientCommand(String);

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
            act.0.write(Message::Ping(Bytes::from_static(b"")));
            act.hb(ctx);

            // client should also check for a timeout here, similar to the
            // server code
        });
    }
}

/// Handle stdin commands
impl Handler<ClientCommand> for ChatClient {
    type Result = ();

    fn handle(&mut self, msg: ClientCommand, _ctx: &mut Context<Self>) {
        self.0.write(Message::Text(msg.0));
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
                self.0.write(Message::Pong(Bytes::from_static(b"")));
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
