use actix::{Actor, Context};
use tonic::transport::Channel;

use crate::message::rtc_message_server::RtcMessage;
// use crate::message::rtc_message_client::RtcMessageClient;
// pub struct MsgRpc {
//     pub cli: RtcMessageClient<Channel>,
// }

// impl Actor for MsgRpc {
//     type Context = Context<Self>;
// }

// #[derive(Default)]
// pub struct RtcServer;

// #[tonic::async_trait]
// impl RtcMessage for RtcServer {
//     type SendMessageStream = ();

//     async fn send_message(
//         &self,
//         request: tonic::Request<crate::message::Mail>,
//     ) -> Result<tonic::Response<Self::SendMessageStream>, tonic::Status> {
//         todo!()
//     }

//     async fn actor_message_state(
//         &self,
//         request: tonic::Request<crate::message::MailEvent>,
//     ) -> Result<tonic::Response<crate::message::MailEvent>, tonic::Status> {
//         todo!()
//     }
// }
