use actix::{Actor, Context};
use tonic::transport::Channel;

use crate::message::rtc_message_client::RtcMessageClient;
pub struct MsgRpc {
    pub cli: RtcMessageClient<Channel>,
}

impl Actor for MsgRpc {
    type Context = Context<Self>;
}
