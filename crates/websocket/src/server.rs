use std::collections::{HashMap, HashSet};
use actix::Recipient;
use diesel::{r2d2::{Pool, ConnectionManager}, PgConnection};
use double_zero_utils::{IpAddr, ConnectionId, LocalUserId, RoomId};
use rand::prelude::ThreadRng;

use crate::messages::WsMessage;


pub struct Session {
  pub addr: Recipient<WsMessage>,
  pub ip: IpAddr,
  pub rooms: Option<HashSet<RoomId>>,
}

pub struct Server {
  /// A map from generated random ID to session addr
  pub sessions: HashMap<ConnectionId, Session>,

  /// A map from community to set of usizes
  pub rooms: HashMap<RoomId, HashSet<LocalUserId>>,

  pub(super) rng: ThreadRng,

  /// The Database Pool
  pub(super) pool: Pool<ConnectionManager<PgConnection>>,

//   /// The Settings
//   pub(super) settings: Settings,

//   /// The Secrets
//   pub(super) secret: Secret,

  // Rate limiting based on rate type and IP addr
  // pub(super) rate_limiter: RateLimit,

//   message_handler: MessageHandlerType,
//   message_handler_crud: MessageHandlerCrudType,

//   activity_queue: QueueHandle,
}