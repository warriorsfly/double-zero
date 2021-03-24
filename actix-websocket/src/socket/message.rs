// use actix::prelude::*;
// use serde::Deserialize;

// use crate::server;

// /// 选择患者
// #[derive(Message, Deserialize)]
// #[rtype(result = "()")]
// pub struct PatientRequest {
//     /// 由websocket session传递调用人员信息
//     pub request_identity: Option<String>,
//     /// 查询类型,比如zjlx=01
//     pub rtype: String,
//     /// 查询zjhm =342612195607131342
//     pub number: String,
// }

// impl Handler<PatientRequest> for server::WinWebsocket {
//     type Result = ();

//     fn handle(&mut self, msg: PatientRequest, ctx: &mut Self::Context) -> Self::Result {
//         //在这里做rpc相关
//         todo!()
//     }
// }

// /// 生命体征数据结构
// #[derive(Deserialize)]
// pub struct Vital {
//     pub code: String,
//     pub name: String,
//     pub value: f32,
// }

// /// 生命体征
// #[derive(Message, Deserialize)]
// #[rtype(result = "()")]
// pub struct VitalSignRequest {
//     /// 由websocket session传递调用人员信息
//     pub request_identity: Option<String>,
//     /// 查询类型,比如zjlx=01
//     pub patient_id: String,
//     /// 查询zjhm =342612195607131342
//     pub vitals: Vec<Vital>,
// }

// impl Handler<VitalSignRequest> for server::WinWebsocket {
//     type Result = ();

//     fn handle(&mut self, msg: VitalSignRequest, ctx: &mut Self::Context) -> Self::Result {
//         //在这里做rpc相关
//         todo!()
//     }
// }
