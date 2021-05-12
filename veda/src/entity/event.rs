use redis::{FromRedisValue, ToRedisArgs};
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

impl ToRedisArgs for Event {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        "subject".write_redis_args(out);
        out.write_arg(&self.subject.as_bytes());
        "act".write_redis_args(out);
        out.write_arg(&self.act.as_bytes());
        "object".write_redis_args(out);
        out.write_arg(&self.object.as_bytes());
    }
}

impl ToRedisArgs for &Event {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        "subject".write_redis_args(out);
        out.write_arg(&self.subject.as_bytes());
        "act".write_redis_args(out);
        out.write_arg(&self.act.as_bytes());
        "object".write_redis_args(out);
        out.write_arg(&self.object.as_bytes());
    }
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
