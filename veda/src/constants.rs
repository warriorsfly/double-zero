use std::time::Duration;

/// js toISOString() in test suit can't handle chrono's default precision
//pub const DATE_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

/// max len of redis stream for each key is 1000
//pub const MAXLEN: StreamMaxlen = StreamMaxlen::Approx(1000);

/// blocking message time milliseconds
pub const BLOCK_MILLIS: usize = 600;
/// polling message time interval
pub const MESSAGE_INTERVAL: Duration = Duration::from_millis(1000);
/// How often heartbeat pings are sent
pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);
/// How long before lack of client response causes a timeout
pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(60);
