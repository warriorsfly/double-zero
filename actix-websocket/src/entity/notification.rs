use redis::FromRedisValue;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Notification {
    pub id: String,
    pub title: String,
    pub content: String,
}

impl FromRedisValue for Notification {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Notification> {
        match *v {
            redis::Value::Data(ref val) => match serde_json::from_slice(val) {
                Err(_) => Err(((redis::ErrorKind::TypeError, "Can't unjson value")).into()),
                Ok(v) => Ok(v),
            },
            _ => Err(((
                redis::ErrorKind::ResponseError,
                "Response type not Dashboard compatible.",
            ))
                .into()),
        }
    }

    fn from_redis_values(items: &[redis::Value]) -> redis::RedisResult<Vec<Self>> {
        Ok(items
            .iter()
            .filter_map(|item| FromRedisValue::from_redis_value(item).ok())
            .collect())
    }
}
