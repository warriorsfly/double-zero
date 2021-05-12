use actix::{Actor, Context, Recipient};
use chrono::Utc;
use tonic::Code;

use super::Trial;
use crate::grpc::{message_server::Message, Msg, MsgStatus, RespMsg};

// #[derive(Default)]
pub struct Seravee {
    redis_addr: Recipient<Trial>,
}

impl Actor for Seravee {
    type Context = Context<Self>;
}

#[tonic::async_trait]
impl Message for Seravee {
    async fn send_msg(
        &self,
        request: tonic::Request<Msg>,
    ) -> Result<tonic::Response<RespMsg>, tonic::Status> {
        let msg = request.into_inner();
        let trail = Trial {
            message: msg.content.clone(),
            receivers: msg.receivers,
        };

        let ids = &self.redis_addr.send(trail).await;
        match ids {
            Ok(ids) => {
                let msgs: Vec<MsgStatus> = ids
                    .into_iter()
                    .map(|str| MsgStatus {
                        message: str.to_owned(),
                        receiver: "".to_string(),
                        action: 0,
                        expire_at: Utc::now().timestamp(),
                    })
                    .collect();

                let resp = RespMsg { states: msgs };

                let response: tonic::Response<RespMsg> = tonic::Response::new(resp);
                Ok(response)
            }
            Err(e) => Err(tonic::Status::new(Code::InvalidArgument, e.to_string())),
        }
    }

    async fn act_msg_state(
        &self,
        request: tonic::Request<MsgStatus>,
    ) -> Result<tonic::Response<MsgStatus>, tonic::Status> {
        todo!()
    }
}
