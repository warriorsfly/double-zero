// use crate::UserOperation;
use actix::{prelude::*, Recipient};
// use lemmy_api_common::{comment::CommentResponse, post::PostResponse};
// use lemmy_db_schema::newtypes::{usize, usize, usize};
use double_zero_utils::{ConnectionId, IpAddr, LocalUserId, CommunityId, TaskId};
use serde::{Deserialize, Serialize};

/// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

/// Message for chat server communications

/// New chat session is created
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
  pub id: ConnectionId,
  pub ip: IpAddr,
}

/// The messages sent to websocket clients
#[derive(Serialize, Deserialize, Message)]
#[rtype(result = "Result<String, std::convert::Infallible>")]
pub struct StandardMessage {
  /// Id of the client session
  pub id: ConnectionId,
  /// Peer message
  pub msg: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SendAllMessage<OP: ToString, Response> {
  pub op: OP,
  pub response: Response,
  pub session_id: Option<ConnectionId>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinUserRoom {
  pub local_user_id: LocalUserId,
  pub id: ConnectionId,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinCommunityRoom {
  pub community_id: CommunityId,
  pub id: ConnectionId,
}



#[derive(Message)]
#[rtype(result = "()")]
pub struct JoinTaskGroup {
  pub task_id: TaskId,
  pub id: ConnectionId,
}

#[derive(Message)]
#[rtype(usize)]
pub struct UsersOnline;

#[derive(Message)]
#[rtype(usize)]
pub struct TaskUsersOnline {
  pub task_id: usize,
}

#[derive(Message)]
#[rtype(usize)]
pub struct CommunityUsersOnline {
  pub community_id: CommunityId,
}

