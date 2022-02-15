use actix::Recipient;
use double_zero_utils::{IpAddr, RoomId};
use std::collections::HashSet;

use crate::messages::RootMessage;

pub struct Session {
    pub addr: Recipient<RootMessage>,
    pub ip: IpAddr,
    pub rooms: Option<HashSet<RoomId>>,
}
