use actix::prelude::*;
use actix_web::{middleware::Logger, web, App, HttpServer};
use rand::{prelude::ThreadRng, Rng};

use std::{
    collections::{HashMap, HashSet},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use crate::handler::{studio_route, vistors_count};

#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

/// 接入websocket服务
#[derive(Message, Debug)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<Message>,
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

/// 显示在线的names
pub struct ListNames;

impl actix::Message for ListNames {
    type Result = Vec<String>;
}

pub struct StudioWebsocket {
    //链接信息
    // sessions.key: websocket session的id
    // sessions.value: websocket 接受参数地址
    sessions: HashMap<usize, Recipient<Message>>,
    //允许一个用户多个链接
    // identities.key:用户名,传递规则不限制,需保证唯一性
    // identities.value:sessions的key
    identities: HashMap<String, HashSet<usize>>,
    rng: ThreadRng,
    visitors: Arc<AtomicUsize>,
}

impl StudioWebsocket {
    pub fn new(count: Arc<AtomicUsize>) -> Self {
        Self {
            sessions: HashMap::with_capacity(1),
            identities: HashMap::with_capacity(1),
            rng: rand::thread_rng(),
            visitors: count,
        }
    }
}

impl StudioWebsocket {
    /// 发送消息到指定name的所有客户端
    fn send_message(&self, identity: &str, message: &str) {
        if let Some(ses) = self.identities.get(identity) {
            for id in ses {
                if let Some(addr) = self.sessions.get(id) {
                    let _ = addr.do_send(Message(message.to_owned()));
                }
            }
        }
    }
}

impl Actor for StudioWebsocket {
    type Context = Context<Self>;
}

impl Handler<Connect> for StudioWebsocket {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        let id = self.rng.gen::<usize>();
        println!("websocket connection {} connected", id);
        self.sessions.insert(id, msg.addr);
        // 新的连接会增加连接数量,不一定会引起用户数量增加
        id
    }
}

impl Handler<Disconnect> for StudioWebsocket {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.id);

        let mut name = None;
        {
            for (session_name, sessions) in &mut self.identities {
                if sessions.contains(&msg.id) {
                    sessions.remove(&msg.id);
                    println!("websocket {:?}", msg);
                    // 当names的value为空,表示当前name下没有websocket连接
                    // name下线,visitor-1
                    if sessions.is_empty() {
                        name = Some(session_name.to_string());
                        self.visitors.fetch_sub(1, Ordering::SeqCst);
                    }
                    break;
                }
            }
        }

        if let Some(name) = name {
            self.identities.remove(&name);
            println!("identity {:?} disconnected", &name);
        }
    }
}

impl Handler<IdentitySession> for StudioWebsocket {
    type Result = ();

    fn handle(&mut self, msg: IdentitySession, _: &mut Self::Context) -> Self::Result {
        if self
            .identities
            .values()
            .any(|sessions| sessions.contains(&msg.id))
        {
            println!("/identity can only used once in one websocket session");
            return;
        }
        if let Some(sessions) = self.identities.get_mut(&msg.name) {
            sessions.insert(msg.id);
        } else {
            //当identities不存在msg.name,添加msg.name到identities中

            println!("identity {:?} connected", msg.name);
            let mut sessions = HashSet::with_capacity(1);
            sessions.insert(msg.id);
            self.identities.insert(msg.name, sessions);
            self.visitors.fetch_add(1, Ordering::SeqCst);
        }
    }
}

impl Handler<ListNames> for StudioWebsocket {
    type Result = MessageResult<ListNames>;

    fn handle(&mut self, _: ListNames, _: &mut Self::Context) -> Self::Result {
        let keys = self
            .identities
            .keys()
            .map(|item| item.to_string())
            .collect();
        MessageResult(keys)
    }
}

pub async fn serv() -> std::io::Result<()> {
    let app_state = Arc::new(AtomicUsize::new(0));
    // Start planet server actor
    let server = StudioWebsocket::new(app_state.clone()).start();

    // Create Http server with websocket support
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .data(app_state.clone())
            .data(server.clone())
            .route("/count/", web::get().to(vistors_count))
            // websocket
            .service(web::resource("/studio/").to(studio_route))
        // static resources
        // .service(fs::Files::new("/static/", "static/"))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
