use std::collections::{HashMap, HashSet};

use actix::Recipient;
use double_zero_utils::IpAddr;
use rand::prelude::ThreadRng;

use crate::messages::RealMessage;


pub struct Session {
  pub addr: Recipient<RealMessage>,
  pub ip: IpAddr,
}

pub struct Server {
  /// A map from generated random ID to session addr
  pub sessions: HashMap<usize, Session>,

  /// A map from community to set of usizes
  pub community_rooms: HashMap<usize, HashSet<usize>>,

  pub mod_rooms: HashMap<usize, HashSet<usize>>,

  /// A map from user id to its connection ID for joined users. Remember a user can have multiple
  /// sessions (IE clients)
  pub(super) user_groups: HashMap<LocalUserId, HashSet<usize>>,

  pub(super) rng: ThreadRng,

  /// The Database Pool
  pub(super) pool: DatabasePool,

  /// The Settings
  pub(super) settings: Settings,

  /// The Secrets
  pub(super) secret: Secret,

  /// Rate limiting based on rate type and IP addr
  pub(super) rate_limiter: RateLimit,

  /// A list of the current captchas
//   pub(super) captchas: Vec<CaptchaItem>,

  message_handler: MessageHandlerType,
  message_handler_crud: MessageHandlerCrudType,

//   activity_queue: QueueHandle,
}