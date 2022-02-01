use std::collections::{HashMap, HashSet};

use diesel::{r2d2::{Pool, ConnectionManager}, PgConnection};
use double_zero_utils::{ConnectionId, LocalUserId, RoomId};

use rand::prelude::ThreadRng;
use session::Session;

pub mod messages;
pub mod session;


pub struct Server {
  /// A map from generated random ID to session addr
  pub sessions: HashMap<ConnectionId, Session>,

  /// A map from community to set of usizes
  pub rooms: HashMap<RoomId, HashSet<LocalUserId>>,

  pub(crate) rng: ThreadRng,

  /// The Database Pool
  pub(crate) pool: Pool<ConnectionManager<PgConnection>>,

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