// js toISOString() in test suit can't handle chrono's default precision
//pub const DATE_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

// max len of redis stream for each key is 1000
//pub const MAXLEN: StreamMaxlen = StreamMaxlen::Approx(1000);

// blocking message time milliseconds
// pub const BLOCK_MILLIS: usize = 600;
// polling message time interval
// pub const MESSAGE_INTERVAL: Duration = Duration::from_millis(1000);
// How often heartbeat pings are sent
// pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);
// How long before lack of client response causes a timeout
// pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(60);



// use double_zero_utils::{DoubleZeroError, UserId};
// use redis::{FromRedisValue, ToRedisArgs};
// use serde::{Deserialize, Serialize};

// use crate::system::WsServer;

// /// device info
// #[derive(Deserialize, Serialize)]
// pub struct Info {
//     device_name: String,
//     factory_name: Option<String>,
//     serial_number: Option<String>,
// }

// impl FromRedisValue for Info {
//     fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
//         match *v {
//             redis::Value::Data(ref val) => match serde_json::from_slice(val) {
//                 Err(_) => Err(((redis::ErrorKind::TypeError, "Can't serialize value")).into()),
//                 Ok(v) => Ok(v),
//             },
//             _ => Err(((
//                 redis::ErrorKind::ResponseError,
//                 "Response type not Dashboard compatible.",
//             ))
//                 .into()),
//         }
//     }
// }

// impl ToRedisArgs for Info {
//     fn write_redis_args<W>(&self, out: &mut W)
//     where
//         W: ?Sized + redis::RedisWrite,
//     {
//         "device_name".write_redis_args(out);
//         let _ = &self.device_name.write_redis_args(out);
//         if let Some(factory_name) = &self.factory_name {
//             "factory_name".write_redis_args(out);
//             factory_name.write_redis_args(out);
//         }

//         if let Some(serial_number) = &self.serial_number {
//             "serial_number".write_redis_args(out);
//             serial_number.write_redis_args(out);
//         }
//     }
// }

// #[derive(Deserialize, Serialize)]
// #[serde(tag = "platform", content = "device")]
// pub enum Platform {
//     Android(Info),
//     Embedded(Info),
//     IPhone(Info),
//     IPad(Info),
//     Macos(Info),
//     Tablet(Info),
//     Web(Info),
//     Windows(Info),
// }

// impl ToRedisArgs for Platform {
//     fn write_redis_args<W>(&self, out: &mut W)
//     where
//         W: ?Sized + redis::RedisWrite,
//     {
//         let (platform, device) = match self {
//             Platform::Android(info) => ("android", info),
//             Platform::Embedded(info) => ("embedded", info),
//             Platform::IPhone(info) => ("iphone", info),
//             Platform::IPad(info) => ("ipad", info),
//             Platform::Macos(info) => ("macos", info),
//             Platform::Tablet(info) => ("tablet", info),
//             Platform::Web(info) => ("web", info),
//             Platform::Windows(info) => ("windows", info),
//         };

//         out.write_arg(b"platform");
//         out.write_arg(platform.as_bytes());
//         out.write_arg(b"device");
//         device.write_redis_args(out);
//     }
// }

// impl FromRedisValue for Platform {
//     fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
//         match *v {
//             redis::Value::Data(ref val) => match serde_json::from_slice(val) {
//                 Err(_) => Err(((redis::ErrorKind::TypeError, "Can't serialize value")).into()),
//                 Ok(v) => Ok(v),
//             },
//             _ => Err(((
//                 redis::ErrorKind::ResponseError,
//                 "Response type not Dashboard compatible.",
//             ))
//                 .into()),
//         }
//     }
// }
