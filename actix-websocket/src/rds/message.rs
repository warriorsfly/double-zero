use actix::prelude::*;
use redis::RedisResult;
use serde::Deserialize;

/// create `online` hashmap in redis,(id,name)
/// create `client$name`
/// 用户上线
#[derive(Message, Deserialize)]
#[rtype(result = "RedisResult<()>")]
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
pub struct RedisMessage {
    pub name: String,
    pub title: String,
    pub content: String,
}
