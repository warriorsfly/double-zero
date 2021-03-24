use actix::prelude::*;
use serde::Deserialize;

/// create `online` hashmap in redis,(id,name)
/// create `client$name`
/// 用户上线
#[derive(Message, Deserialize)]
#[rtype(result = "()")]
pub struct Online {
    /// websocket session id
    pub id: usize,
    ///
    pub name: String,
}

/// 用户下线
#[derive(Message, Deserialize)]
#[rtype(result = "()")]
pub struct Offline {
    /// websocket session id
    pub id: usize,
}
/// 缓存数据
#[derive(Message, Deserialize)]
#[rtype(result = "()")]
pub struct RedStoreMessaging {
    pub name: String,
    pub title: String,
    pub content: String,
}
