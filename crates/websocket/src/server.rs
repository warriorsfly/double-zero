use std::collections::{HashMap, HashSet};
use actix::prelude::*;
use diesel::{r2d2::{Pool, ConnectionManager}, PgConnection};
use double_zero_utils::{ConnectionId, UserId, RoomId};
use rand::{prelude::ThreadRng, Rng};
use tracing::info;
use crate::messages::{Connect, Disconnect};

use super::session::Session;

pub struct ChatServer {
  /// A map from generated random ID to session addr
  pub sessions: HashMap<ConnectionId, Session>,

  /// A map from community to set of usizes
  pub rooms: HashMap<RoomId, HashSet<UserId>>,

  pub(crate) rng: ThreadRng,

  /// The Database Pool
  pub(crate) pool: Pool<ConnectionManager<PgConnection>>,

//   /// The Settings
//   pub(super) settings: Settings,

//   /// The Secrets
//   pub(super) secret: Secret,

  // Rate limiting based on rate type and IP addr
  // pub(super) rate_limiter: RateLimit,
}


/// Make actor from `ChatServer`
impl Actor for ChatServer {
  /// We are going to use simple Context, we just need ability to communicate
  /// with other actors.
  type Context = Context<Self>;
}

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
impl Handler<Connect> for ChatServer {
  type Result = ConnectionId;

  fn handle(&mut self, msg: Connect, _ctx: &mut Context<Self>) -> Self::Result {
    // register session with random id
    let id = self.rng.gen::<usize>();
    info!("{} joined", &msg.ip);

    self.sessions.insert(
      id,
      Session {
        addr: msg.addr,
        ip: msg.ip,
        rooms:None,
      },
    );

    id
  }
}

/// Handler for Disconnect message.
impl Handler<Disconnect> for ChatServer {
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