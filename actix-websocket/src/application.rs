use actix::prelude::*;
use rand::{prelude::ThreadRng, Rng};

use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

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

pub struct Application {
    //链接信息
    // sessions.key: websocket session的id
    // sessions.value: websocket 接受参数地址
    sessions: HashMap<usize, Recipient<Messaging>>,
    rng: ThreadRng,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            sessions: HashMap::with_capacity(1),
            rng: rand::thread_rng(),
        }
    }
}

impl Application {
    /// 发送消息到指定name的所有客户端
    fn send_message(&self, id: usize, message: &str) {
        if let Some(addr) = self.sessions.get(&id) {
            let _ = addr.do_send(Messaging(message.to_owned()));
        }
    }
}

impl Actor for Application {
    type Context = Context<Self>;
}

impl Handler<Connect> for Application {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        let id = self.rng.gen::<usize>();
        println!("websocket connection {} connected", id);
        self.sessions.insert(id, msg.addr);
        // 新的连接会增加连接数量,不一定会引起用户数量增加
        id
    }
}

impl Handler<Disconnect> for Application {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.id);
        println!("identity {:?} disconnected", &msg.id);
    }
}

impl Handler<RedisMessage> for Application {
    type Result = ();

    fn handle(&mut self, msg: RedisMessage, _: &mut Self::Context) -> Self::Result {
        self.send_message(msg.id, &msg.msg);
    }
}
