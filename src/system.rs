use std::collections::{HashMap, HashSet};
use actix::prelude::*;
use double_zero_utils::{ConnectionId, UserId, RoomId, pool::DbPool, IpAddr};
use rand::{prelude::ThreadRng, Rng};
use tracing::info;
use crate::messages::{Connect, Disconnect, JoinRoom, WsMessage};

pub struct Session {
  pub addr: Recipient<WsMessage>,
  pub ip: IpAddr,
}
pub struct DoubleZeroSystem {
  pub(crate) rng: ThreadRng,

  pub(crate) pool:DbPool,
  /// A map from generated random ID to session addr
  pub sessions: HashMap<ConnectionId, Session>,

  /// A map from community to set of usizes
  pub rooms: HashMap<RoomId, HashSet<ConnectionId>>,
}


/// Make actor from `ChatServer`
impl Actor for DoubleZeroSystem {
  /// We are going to use simple Context, we just need ability to communicate
  /// with other actors.
  type Context = Context<Self>;
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for DoubleZeroSystem {
  type Result = ConnectionId;

  fn handle(&mut self, msg: Connect, _ctx: &mut Context<Self>) -> Self::Result {
    // register session with random id
    let id = self.rng.gen::<usize>();
    info!("{} joined", &msg.ip);

    // self.sessions.insert(
    //   id,
    //   Session {
    //     addr: msg.addr,
    //     ip: msg.ip,
    //     rooms:None,
    //   },
    // );

    id
  }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for DoubleZeroSystem {
  type Result = ();

  fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
    // Remove connections from sessions and all 3 scopes
    if self.sessions.remove(&msg.websocket_id).is_some() {
      for sessions in self.rooms.values_mut() {
        sessions.remove(&msg.websocket_id);
      }

  
    }
  }
}

/// Handler for JoinRoom message.
impl Handler<JoinRoom> for DoubleZeroSystem {
  type Result = ();

  fn handle(&mut self, msg: JoinRoom, _: &mut Context<Self>) {
    // JoinRoom
    // let room = self.rooms.get_mut(&msg.room_id).unwa
  }
}