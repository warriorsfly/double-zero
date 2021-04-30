use redis::FromRedisValue;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Event {
    /// some one
    pub subject: String,
    ///`{"method":"notify",content:{"id":"1","title":"ti","content":"nothing"}}`
    pub act: String,
    /// any one
    pub object: String,
}

impl FromRedisValue for Event {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Event> {
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
