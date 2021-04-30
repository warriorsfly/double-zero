// use crate::message::msg_box_server::MsgBox;
// use actix::{Actor, Context};
// use tonic::transport::Channel;
// // use crate::message::rtc_message_client::RtcMessageClient;
// // pub struct MsgRpc {
// //     pub cli: RtcMessageClient<Channel>,
// // }

// // #[derive(Default)]
// pub struct Grpc {
//     redis_addr: Recipient<WsMessage>,
// }

// impl Actor for Grpc {
//     type Context = Context<Self>;
// }

// #[tonic::async_trait]
// impl MsgBox for Grpc {
//     async fn send_msg(
//         &self,
//         request: tonic::Request<crate::message::Msg>,
//     ) -> Result<tonic::Response<crate::message::MsgStatusEvent>, tonic::Status> {
//         let msg = request.into();
//     }

//     async fn act_message_state(
//         &self,
//         request: tonic::Request<crate::message::MsgStatusEvent>,
//     ) -> Result<tonic::Response<crate::message::MsgStatusEvent>, tonic::Status> {
//         todo!()
//     }
// }
