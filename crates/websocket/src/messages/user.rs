use actix::Message;
use double_zero_utils::{ConnectionId, RoomId};
use serde::{Serialize, Deserialize};

/// The messages sent to websocket clients
#[derive(Serialize, Deserialize, Message)]
#[rtype(result = "Result<String, std::convert::Infallible>")]
pub struct DzMessage {
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