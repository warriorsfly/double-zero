use actix::{Actor, Context};
use tonic::transport::Channel;

use crate::message::{actix_message_client::ActixMessageClient, Message, Receiver};
pub struct MsgRpc {
    pub cli: ActixMessageClient<Channel>,
}

impl Actor for MsgRpc {
    type Context = Context<Self>;
}
