use actix::{Recipient, Addr};
use actix_web_actors::ws;
use double_zero_utils::{IpAddr, RoomId};
use std::collections::HashSet;

use crate::{messages::GnMessage, server::TransAmSystem};

pub struct Session {
    pub id: u32,
    pub hb: Instant,
    pub addr: Addr<TransAmSystem>,
    pub ip: IpAddr,
    pub rooms: Option<HashSet<RoomId>>,
}

impl Actor for Session {
    type Context = ws::WebsocketContext<Self>;
    
    
}
