use actix::Recipient;
use double_zero_utils::{IpAddr, RoomId};
use std::collections::HashSet;

use crate::messages::RealTimeMessage;

pub struct Session {
    pub addr: Recipient<RealTimeMessage>,
    pub ip: IpAddr,
    pub rooms: Option<HashSet<RoomId>>,
}
