
use serde::{Serialize, Deserialize};

use crate::schema::users;
#[derive(Clone, Queryable, Identifiable, PartialEq, Debug, Serialize, Deserialize)]
#[diesel(table_name = users)]
pub struct User {
  pub id: i32,
  pub name: String,
  pub display_name: Option<String>,
  pub avatar: Option<String>,
  pub updated_at: Option<chrono::NaiveDateTime>,
  pub bio: Option<String>,
  pub local: bool,
  pub private_key: Option<String>,
  pub public_key: String,
  pub last_refreshed_at: chrono::NaiveDateTime,
  pub deleted: bool,
  pub inbox_url: String,
  pub shared_inbox_url: Option<String>,
  pub admin: bool,
  pub bot_account: bool,
}