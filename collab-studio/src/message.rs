use actix::prelude::*;
use serde::Deserialize;

use crate::server;

/// 选择患者
#[derive(Message, Deserialize)]
#[rtype(result = "()")]
pub struct PatientRequest {
    pub request_identity: Option<String>,
    /// 查询类型,比如zjlx=01
    pub rtype: String,
    /// 查询zjhm =342612195607131342
    pub number: String,
}

impl Handler<PatientRequest> for server::StudioWebsocket {
    type Result = ();

    fn handle(&mut self, msg: PatientRequest, ctx: &mut Self::Context) -> Self::Result {
        todo!()
    }
}
