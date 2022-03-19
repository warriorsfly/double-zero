// use crate::UserOperation;
use actix::{prelude::*, Recipient};
use double_zero_utils::{ConnectionId, IpAddr, RoomId};
use serde::{Deserialize, Serialize};

/// trans am server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

/// Message for chat server communications
/// New trans am session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub ip: IpAddr,
}

/// Session is disconnected
#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub websocket_id: ConnectionId,
    pub ip: IpAddr,
}


/// The messages sent to websocket clients
#[derive(Serialize, Deserialize, Message)]
#[rtype(result = "Result<String, std::convert::Infallible>")]
pub struct TextMessage {
    /// Id of the client session
    pub websocket_id: ConnectionId,
    /// Peer message
    pub msg: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinRoom {
    pub room_id: RoomId,
    pub websocket_id: ConnectionId,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct LeaveRoom {
    pub room_id: RoomId,
    pub websocket_id: ConnectionId,
}


