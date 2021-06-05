use std::net::SocketAddr;

use actix::{Actor, Context, Recipient};
use chrono::Utc;
use tonic::Code;

use super::Trial;
use crate::{
    entity::{Event, MessageFlow},
    grpc::{
        message_source_server::MessageSource, FlowAction, FlowResource, FlowStatus,
        MessageFlow as RpcFlow,
    },
};

impl Into<MessageFlow> for RpcFlow {
    fn into(self) -> MessageFlow {
        match self.flow_type.as_str() {
            "event" => {
                let content: Event = serde_json::from_str(&self.content).expect("Unreadable event");
                MessageFlow::Evn(content)
            }
            "text" => MessageFlow::Text(self.content),
            "url" => MessageFlow::Url(self.content),
            _ => panic!("error message type"),
        }
    }
}
#[derive(Clone)]
pub struct Seravee {
    pub addr: SocketAddr,
    pub redis_addr: Recipient<Trial>,
}

impl Actor for Seravee {
    type Context = Context<Self>;
}

#[tonic::async_trait]
impl MessageSource for Seravee {
    async fn send_flow(
        &self,
        request: tonic::Request<FlowResource>,
    ) -> Result<tonic::Response<FlowAction>, tonic::Status> {
        let msg = request.into_inner();
        let content = msg.message.unwrap();
        let trail = Trial {
            message: content.into(),
            receivers: msg.receivers,
        };

        let ids = &self.redis_addr.send(trail).await;
        match ids {
            Ok(ids) => {
                let msgs: Vec<FlowStatus> = ids
                    .into_iter()
                    .map(|str| FlowStatus {
                        message: str.to_owned().1,
                        receiver: str.to_owned().0,
                        action: 0,
                        expire_at: Utc::now().timestamp(),
                    })
                    .collect();

                let resp = FlowAction { states: msgs };

                let response: tonic::Response<FlowAction> = tonic::Response::new(resp);
                Ok(response)
            }
            Err(e) => Err(tonic::Status::new(Code::InvalidArgument, e.to_string())),
        }
    }

    async fn act_flow(
        &self,
        request: tonic::Request<FlowStatus>,
    ) -> Result<tonic::Response<FlowStatus>, tonic::Status> {
        todo!()
    }
}
