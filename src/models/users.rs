use diesel::{prelude::{Insertable, Queryable}, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
  pub id: i32,
  pub username: String,
  pub skin: Option<String>,
  pub cape: Option<String>,
}