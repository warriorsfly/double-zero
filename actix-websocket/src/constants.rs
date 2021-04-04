use std::time::Duration;

use redis::streams::StreamMaxlen;

/// js toISOString() in test suit can't handle chrono's default precision
pub const DATE_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

/// 默认`通知` 通道
pub const STREAM_MESSAGES: &str = "stream-messages";
// /// Android `通知` 通道
// pub const CHANNEL_MESSAGES_ANDROID: &str = "stream-messages-android";
// /// Ios `通知` 通道
// pub const CHANNEL_MESSAGES_IOS: &str = "stream-messages-ios";

/// Redis Stream监听的`channels`
pub const CHANNELS: &[&str] = &[
    STREAM_MESSAGES,
    // CHANNEL_MESSAGES_ANDROID,
    // CHANNEL_MESSAGES_IOS,
];

/// max len of group stream is 1000000
pub const MAXLEN: StreamMaxlen = StreamMaxlen::Approx(1000000);

/// 多stream监听需要使用 block
pub const BLOCK_MILLIS: usize = 800;
/// 轮训 xgroup read的时间
pub const MESSAGE_INTERVAL: Duration = Duration::from_millis(1000);
/// How often heartbeat pings are sent
pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(60);
/// How long before lack of client response causes a timeout
pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(120);
