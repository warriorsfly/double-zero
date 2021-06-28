use std::net::SocketAddr;

use actix::{Actor, Context, Recipient};
use chrono::Utc;
use tonic::Code;

use super::Trial;
use crate::{
    activity::{self, activity_source_server::ActivitySource},
    entity::Activity,
};

impl Into<Activity> for activity::Activity {
    fn into(self) -> Activity {
        Activity {
            activity_type: self.activity_type,
            activity: self.content,
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
impl ActivitySource for Seravee {
    async fn active(
        &self,
        request: tonic::Request<activity::Message>,
    ) -> Result<tonic::Response<activity::States>, tonic::Status> {
        let msg = request.into_inner();
        let content = msg.message.unwrap();
        let trail = Trial {
            message: content.into(),
            receivers: msg.receivers,
        };

        let ids = &self.redis_addr.send(trail).await;
        match ids {
            Ok(ids) => {
                let msgs: Vec<activity::Status> = ids
                    .into_iter()
                    .map(|str| activity::Status {
                        message: str.to_owned().1,
                        receiver: str.to_owned().0,
                        action: 0,
                        expire_at: Utc::now().timestamp(),
                    })
                    .collect();

                let resp = activity::States { states: msgs };

                let response: tonic::Response<activity::States> = tonic::Response::new(resp);
                Ok(response)
            }
            Err(e) => Err(tonic::Status::new(Code::InvalidArgument, e.to_string())),
        }
    }

    async fn act_flow(
        &self,
        _request: tonic::Request<activity::Status>,
    ) -> Result<tonic::Response<activity::Status>, tonic::Status> {
        todo!()
    }

  
}
