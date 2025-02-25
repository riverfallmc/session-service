use adjust::{database::{postgres::Postgres, Database}, response::HttpResult};
use axum::Json;
use diesel::prelude::*;
use crate::{schema::skincape_cache::dsl::*, service::fs::FileSystemService};

pub struct SkinCapeCacheRepository;

impl SkinCapeCacheRepository {
  pub fn add(db: &mut Database<Postgres>, file_name: String) -> HttpResult<()> {
    diesel::insert_into(skincape_cache)
      .values((name.eq(&file_name), user_count.eq(1)))
      .on_conflict(name)
      .do_update()
      .set(user_count.eq(user_count + 1))
      .execute(db)?;

    Ok(Json(()))
  }

  pub async fn remove(db: &mut Database<Postgres>, file_name: String) -> HttpResult<()> {
    let count: i64 = skincape_cache
      .select(user_count)
      .filter(name.eq(&file_name))
      .first::<i64>(db)
      .optional()?
      .unwrap_or(0);

    match count.cmp(&1) {
      std::cmp::Ordering::Greater => {
        diesel::update(skincape_cache.filter(name.eq(&file_name)))
          .set(user_count.eq(user_count - 1))
          .execute(db)?;
      }
      std::cmp::Ordering::Equal => {
        diesel::delete(skincape_cache.filter(name.eq(&file_name)))
          .execute(db)?;

        #[allow(unused)]
        FileSystemService::remove(file_name).await?;
      }
      _ => {}
    }

    Ok(Json(()))
  }
}
