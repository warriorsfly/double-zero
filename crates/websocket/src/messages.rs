// use crate::UserOperation;
use actix::{prelude::*, Recipient};
// use lemmy_api_common::{comment::CommentResponse, post::PostResponse};
// use lemmy_db_schema::newtypes::{CommunityId, LocalUserId, PostId};
use double_zero_utils::{ConnectionId, IpAddr};
use serde::{Deserialize, Serialize};

/// Chat server sends this messages to session
#[derive(Message)]
#[rtype(result = "()")]
pub struct RealMessage(pub String);

/// Message for chat server communications

/// New chat session is created
#[derive(Message)]
#[rtype(usize)]
pub struct Connect {
  pub addr: Recipient<RealMessage>,
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
  pub websocket_id: Option<ConnectionId>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct SendPost<OP: ToString> {
  pub op: OP,
  pub post: PostResponse,
  pub websocket_id: Option<ConnectionId>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub(crate) struct SendComment<OP: ToString> {
  pub op: OP,
  pub comment: CommentResponse,
  pub websocket_id: Option<ConnectionId>,
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
  pub post_id: PostId,
  pub id: ConnectionId,
}

#[derive(Message)]
#[rtype(usize)]
pub struct GetUsersOnline;

#[derive(Message)]
#[rtype(usize)]
pub struct GetTaskUsersOnline {
  pub task_id: PostId,
}

#[derive(Message)]
#[rtype(usize)]
pub struct GetCommunityUsersOnline {
  pub community_id: CommunityId,
}

