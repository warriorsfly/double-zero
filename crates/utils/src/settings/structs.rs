
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, Ipv4Addr};

#[derive(Debug, Deserialize, Serialize, Clone)]
// #[serde(default)]
pub struct Settings {
  // #[default(Some(RateLimitConfig::default()))]
  /// rate limits for various user actions, by user ip
  pub rate_limit: Option<RateLimitConfig>,
  /// Email sending configuration. All options except login/password are mandatory
  // #[default(None)]
  pub email: Option<EmailConfig>,
  /// Parameters for automatic configuration of new instance (only used at first start)
  // #[default(None)]
  /// the domain name of your instance (mandatory)
  // #[default("unset")]
  // #[doku(example = "example.com")]
  pub hostname: String,
  /// Address where lemmy should listen for incoming requests
  // #[default(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)))]
  // #[doku(as = "String")]
  pub bind: IpAddr,
  /// Port where lemmy should listen for incoming requests
  // #[default(8536)]
  pub port: u16,
  /// Whether the site is available over TLS. Needs to be true for federation to work.
  // #[default(true)]
  pub tls_enabled: bool,
  /// Address where pictrs is available (for image hosting)
  // #[default(None)]
  pub pictrs_url: Option<String>,
  // #[default(None)]
  pub slur_filter: Option<String>,
  /// Maximum length of local community and user names
  // #[default(20)]
  pub actor_name_max_length: usize,
  /// Maximum number of HTTP requests allowed to handle a single incoming activity (or a single object fetch through the search).
  // #[default(25)]
  pub http_fetch_retry_limit: i32,

  // #[default(None)]
  pub opentelemetry_url: Option<String>,
}


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EmailConfig {
  /// Hostname and port of the smtp server
  pub smtp_server: String,
  /// Login name for smtp server
  pub smtp_login: Option<String>,
  /// Password to login to the smtp server
  pub smtp_password: Option<String>,
  /// Address to send emails from, eg "warriorsfly@gmail.com"
  pub smtp_from_address: String,
  /// Whether or not smtp connections should use tls
  pub use_tls: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RateLimitConfig {
  /// Maximum number of messages created in interval
  // #[default(180)]
  pub message: i32,
  /// Interval length for message limit, in seconds
  // #[default(60)]
  pub message_per_second: i32,
  /// Maximum number of posts created in interval
  // #[default(6)]
  pub post: i32,
  /// Interval length for post limit, in seconds
  // #[default(600)]
  pub post_per_second: i32,
  /// Maximum number of registrations in interval
  // #[default(3)]
  pub register: i32,
  /// Interval length for registration limit, in seconds
  // #[default(3600)]
  pub register_per_second: i32,
  /// Maximum number of image uploads in interval
  // #[default(6)]
  pub image: i32,
  /// Interval length for image uploads, in seconds
  // #[default(3600)]
  pub image_per_second: i32,
  /// Maximum number of comments created in interval
  // #[default(6)]
  pub comment: i32,
  /// Interval length for comment limit, in seconds
  // #[default(600)]
  pub comment_per_second: i32,
}

