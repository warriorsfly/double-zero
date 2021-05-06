use std::collections::HashMap;

use redis::FromRedisValue;
use serde::{Deserialize, Serialize};

/// device info
#[derive(Deserialize, Serialize)]
pub struct Info {
    device_name: String,
    factory_name: Option<String>,
    serial_number: Option<String>,
}

impl FromRedisValue for Info {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        match *v {
            redis::Value::Data(ref val) => match serde_json::from_slice(val) {
                Err(_) => Err(((redis::ErrorKind::TypeError, "Can't serialize value")).into()),
                Ok(v) => Ok(v),
            },
            _ => Err(((
                redis::ErrorKind::ResponseError,
                "Response type not Dashboard compatible.",
            ))
                .into()),
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "platform", content = "device")]
pub enum Platform {
    Android(Info),
    Embedded(Info),
    IPhone(Info),
    IPad(Info),
    Macos(Info),
    // SmartWatch(Info),
    Tablet(Info),
    Web(Info),
    Windows(Info),
}

impl FromRedisValue for Platform {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        match *v {
            redis::Value::Data(ref val) => match serde_json::from_slice(val) {
                Err(_) => Err(((redis::ErrorKind::TypeError, "Can't serialize value")).into()),
                Ok(v) => Ok(v),
            },
            _ => Err(((
                redis::ErrorKind::ResponseError,
                "Response type not Dashboard compatible.",
            ))
                .into()),
        }
    }
}

/// Gandum meister
#[derive(Deserialize, Serialize)]
pub struct Meister {
    /// identity
    username: String,
    // token: Vec<String>,
    // tags: Vec<String>,
    sessions: HashMap<usize, Platform>,
}

impl FromRedisValue for Meister {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
        match *v {
            redis::Value::Data(ref val) => match serde_json::from_slice(val) {
                Err(_) => Err(((redis::ErrorKind::TypeError, "Can't serialize value")).into()),
                Ok(v) => Ok(v),
            },
            _ => Err(((
                redis::ErrorKind::ResponseError,
                "Response type not Dashboard compatible.",
            ))
                .into()),
        }
    }
}

// impl FromRedisValue for Meister {
//     fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Self> {
//         let
//     }
// }

// /// 存储标签和Meister的关系
// pub struct Halo {
//     pub halo: String,
//     pub meister: String,
// }
// /// 存储token和Meister的关系
// #[derive(Deserialize, Serialize)]
// pub struct Raiser {
//     pub tag: String,
//     pub meister: String,
// }
