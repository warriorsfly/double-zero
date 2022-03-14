use actix_web::web::{Data, Json};
use double_zero_utils::{DoubleZeroError, UserId};
use redis::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};

use crate::server::TransAmSystem;

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
        let _ = &self.device_name.write_redis_args(out);
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
            Platform::Android(info) => ("android", info),
            Platform::Embedded(info) => ("embedded", info),
            Platform::IPhone(info) => ("iphone", info),
            Platform::IPad(info) => ("ipad", info),
            Platform::Macos(info) => ("macos", info),
            Platform::Tablet(info) => ("tablet", info),
            Platform::Web(info) => ("web", info),
            Platform::Windows(info) => ("windows", info),
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

/// device info
#[derive(Deserialize, Serialize)]
pub struct User {
    id: UserId,
    name: String,
    platform: Platform,
}

#[derive(Deserialize, Serialize)]
pub struct UserLoginForm {
    name: String,
    password: String,
    platform: Platform,
}

pub async fn login(am:Data<TransAmSystem>,form:Json<UserLoginForm>)->Result<User,DoubleZeroError>{

}
