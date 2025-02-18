use diesel::{prelude::{Insertable, Queryable}, Selectable};
use serde::{Deserialize, Serialize};
use crate::schema::skincape_cache;

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = skincape_cache)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Session {
  pub name: String,
  pub user_count: i64
}