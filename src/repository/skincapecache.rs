use dixxxie::{connection::DbPooled, response::HttpResult};
use diesel::prelude::*;
use crate::schema::skincape_cache::dsl::*;

pub struct SkinCapeCacheRepository;

impl SkinCapeCacheRepository {
  pub fn add(db: &mut DbPooled, file_name: String) -> HttpResult<()> {
    diesel::insert_into(skincape_cache)
      .values((name.eq(&file_name), user_count.eq(1)))
      .on_conflict(name)
      .do_update()
      .set(user_count.eq(user_count + 1))
      .execute(db)?;

    Ok(())
  }

  pub fn remove(db: &mut DbPooled, file_name: String) -> HttpResult<()> {
    let count: i64 = skincape_cache
      .select(user_count)
      .filter(name.eq(&file_name))
      .first::<i64>(db)
      .optional()?
      .unwrap_or(0);

    if count > 1 {
      diesel::update(skincape_cache.filter(name.eq(&file_name)))
        .set(user_count.eq(user_count - 1))
        .execute(db)?;
    } else {
      diesel::delete(skincape_cache.filter(name.eq(&file_name)))
        .execute(db)?;
    }

    Ok(())
  }
}
