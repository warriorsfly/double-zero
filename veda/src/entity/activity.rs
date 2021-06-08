use redis::{FromRedisValue, ToRedisArgs};
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
pub struct Activity {
    /// event message
    pub activity_type: String,
    pub activity: String,
}

impl ToRedisArgs for &Activity {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        "activity_type".write_redis_args(out);
        self.activity_type.write_redis_args(out);
        "activity".write_redis_args(out);
        self.activity.write_redis_args(out);
    }
}

// impl ToRedisArgs for &Activity {
//     fn write_redis_args<W>(self, out: &mut W)
//     where
//         W: ?Sized + redis::RedisWrite,
//     {
//         &self.write_redis_args(out);
//     }
// }

impl FromRedisValue for Activity {
    fn from_redis_value(v: &redis::Value) -> redis::RedisResult<Activity> {
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
