// use crate::UserOperation;
use actix::{prelude::*, Recipient};
// use double-zero-api_common::{comment::CommentResponse, post::PostResponse};
// use double-zero-db_schema::newtypes::{usize, usize, usize};
use double_zero_utils::{ConnectionId, IpAddr, LocalUserId, RoomId};
use serde::{Deserialize, Serialize};

/// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct RealTimeMessage(pub String);

/// Message for chat server communications
/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
    pub addr: Recipient<RealTimeMessage>,
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
pub struct StandardMessage {
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
#[rtype(usize)]
pub struct RoomUsers;

#[derive(Message)]
#[rtype(usize)]
pub struct RoomOnlineUsers {
    pub room_id: usize,
}