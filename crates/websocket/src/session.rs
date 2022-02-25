use actix::Recipient;
use double_zero_utils::{IpAddr, RoomId};
use std::collections::HashSet;

use crate::messages::GnMessage;

pub struct Session {
    pub addr: Recipient<GnMessage>,
    pub ip: IpAddr,
    pub rooms: Option<HashSet<RoomId>>,
}
