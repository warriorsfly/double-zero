use std::time::Duration;

use redis::streams::StreamMaxlen;

/// 默认`通知` 通道
pub const CHANNEL_MESSAGES: &str = "channel-messages";
// /// Android `通知` 通道
// pub const CHANNEL_MESSAGES_ANDROID: &str = "channel-messages-android";
// /// Ios `通知` 通道
// pub const CHANNEL_MESSAGES_IOS: &str = "channel-messages-ios";

/// Redis Stream监听的`channels`
pub const CHANNELS: &[&str] = &[
    CHANNEL_MESSAGES,
    // CHANNEL_MESSAGES_ANDROID,
    // CHANNEL_MESSAGES_IOS,
];

/// 最大允许消息100000条,按照一个地区1000个医生算,每个人可以存储100条消息
pub const MAXLEN: StreamMaxlen = StreamMaxlen::Approx(100000);

/// 多stream监听需要使用 block
pub const BLOCK_MILLIS: usize = 200;

pub const MESSAGE_INTERVAL: Duration = Duration::from_millis(500);
/// How often heartbeat pings are sent
pub const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(60);
/// How long before lack of client response causes a timeout
pub const CLIENT_TIMEOUT: Duration = Duration::from_secs(120);
