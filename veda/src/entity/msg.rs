use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct PushMessage {
    /// 消息id
    pub id: String,
    /// 消息发送者
    pub sender: String,
    /// 消息内容(可以还是一个消息体)
    pub content: String,
    /// 消息接收者
    pub receivers: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct MessageAction {
    /// 消息id
    pub id: String,
    /// 消息发送者
    pub sender: String,
    /// 消息内容(可以还是一个消息体)
    pub content: String,
    /// 消息接收者
    pub receivers: Vec<String>,
}
