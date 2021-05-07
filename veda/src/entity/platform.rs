use redis::{FromRedisValue, ToRedisArgs};
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

impl ToRedisArgs for Info {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        "device_name".write_redis_args(out);
        &self.device_name.write_redis_args(out);
        if let Some(factory_name) = &self.factory_name {
            "factory_name".write_redis_args(out);
            factory_name.write_redis_args(out);
        }

        if let Some(serial_number) = &self.serial_number {
            "serial_number".write_redis_args(out);
            serial_number.write_redis_args(out);
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
    Tablet(Info),
    Web(Info),
    Windows(Info),
}

impl ToRedisArgs for Platform {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        let (platform, device) = match self {
            Platform::Android(info) => ("Android", info),
            Platform::Embedded(info) => ("Embedded", info),
            Platform::IPhone(info) => ("IPhone", info),
            Platform::IPad(info) => ("IPad", info),
            Platform::Macos(info) => ("Macos", info),
            Platform::Tablet(info) => ("Tablet", info),
            Platform::Web(info) => ("Web", info),
            Platform::Windows(info) => ("Windows", info),
        };

        out.write_arg(b"platform");
        out.write_arg(platform.as_bytes());
        out.write_arg(b"device");
        device.write_redis_args(out);
    }
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

// /// Gandum meister
// #[derive(Deserialize, Serialize)]
// pub struct Meister {
//     /// identity
//     pub username: String,
//     // token: Vec<String>,
//     // tags: Vec<String>,
//     pub sessions: HashMap<usize, Platform>,
// }

// impl FromRedisValue for Meister {
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

// impl ToRedisArgs for Meister {
//     fn write_redis_args<W>(&self, out: &mut W)
//     where
//         W: ?Sized + redis::RedisWrite,
//     {
//         out.write_arg(b"username");
//         out.write_arg(self.username.as_bytes());
//         out.write_arg(b"sessions");
//         out.write_arg(&self.sessions);
//         // self.sessions.write_redis_args(out);
//     }
// }

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
