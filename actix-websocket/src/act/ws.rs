use actix::prelude::*;
use rand::{prelude::ThreadRng, Rng};
use redis::Client;

use std::{collections::HashMap, sync::Arc};
#[derive(Message)]
#[rtype(result = "()")]
pub struct Messaging(pub String);

/// 接入websocket服务
#[derive(Message, Debug)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Messaging>,
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
    redis: Arc<Client>,
    //链接信息
    // soc_sessions.key: websocket session的id
    // soc_sessions.value: websocket 接受参数地址
    sessions: HashMap<usize, Recipient<Messaging>>,
    // red_sessions.key: redis steam session的id
    rng: ThreadRng,
}

impl Default for Websocket {
    fn default() -> Self {
        Self {
            redis: Arc::new(Client::open("127.0.0.1").expect("error redis url")),
            sessions: HashMap::with_capacity(1),
            rng: rand::thread_rng(),
        }
    }
}

impl Websocket {
    /// 发送消息到指定name的所有客户端
    fn send_message(&self, id: usize, message: &str) {
        if let Some(addr) = self.sessions.get(&id) {
            let _ = addr.do_send(Messaging(message.to_owned()));
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
        println!("identity {:?} disconnected", &msg.id);
    }
}

impl Handler<RedisMessage> for Websocket {
    type Result = ();

    fn handle(&mut self, msg: RedisMessage, _: &mut Self::Context) -> Self::Result {
        self.send_message(msg.id, &msg.msg);
    }
}
