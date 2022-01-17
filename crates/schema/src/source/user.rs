
use serde::{Serialize, Deserialize};

use crate::schema::user_;
#[derive(Clone, Queryable, Identifiable, PartialEq, Debug, Serialize, Deserialize)]
#[table_name = "user_"]
pub struct User {
  pub id: i32,
  pub name: String,
  pub display_name: Option<String>,
  pub avatar: Option<String>,
  pub banned: bool,
  pub published: chrono::NaiveDateTime,
  pub updated: Option<chrono::NaiveDateTime>,
  pub actor_id: String,
  pub bio: Option<String>,
  pub local: bool,
  pub private_key: Option<String>,
  pub public_key: String,
  pub last_refreshed_at: chrono::NaiveDateTime,
  pub banner: Option<String>,
  pub deleted: bool,
  pub inbox_url: String,
  pub shared_inbox_url: Option<String>,
  pub matrix_user_id: Option<String>,
  pub admin: bool,
  pub bot_account: bool,
  pub ban_expires: Option<chrono::NaiveDateTime>,
}