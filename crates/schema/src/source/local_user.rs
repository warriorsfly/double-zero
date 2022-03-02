
use crate::schema::local_users;
use serde::{Deserialize, Serialize};

#[derive(Clone, Queryable, Identifiable, PartialEq, Debug, Serialize, Deserialize)]
#[diesel(table_name =local_users)]
pub struct LocalUser {
  pub id: i32,
  pub user_id: i32,
  pub password_encrypted: String,
  pub email: Option<String>,
  pub show_avatars: bool,
  pub send_notifications_to_email: bool,
  pub validator_time: chrono::NaiveDateTime,
  pub show_bot_accounts: bool,
  pub email_verified: bool,
  pub accepted_application: bool,
}
