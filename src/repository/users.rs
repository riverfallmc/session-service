use dixxxie::{connection::DbPooled, response::HttpResult};
use diesel::prelude::*;
use crate::models::users::User;
use crate::schema::users::dsl::*;
use crate::repository::skincapecache::SkinCapeCacheRepository;

pub struct UsersRepository;

impl UsersRepository {
  async fn ensure_user_exists(db: &mut DbPooled, user_name: &str) -> HttpResult<i32> {
    if let Ok(user_id) = users
      .select(id)
      .filter(username.eq(user_name))
      .first::<i32>(db)
    {
      return Ok(user_id);
    }

    let new_user = diesel::insert_into(users)
      .values(username.eq(user_name))
      .returning(id)
      .get_result::<i32>(db)?;

    Ok(new_user)
  }

  #[allow(unused)]
  pub async fn get_by_id(db: &mut DbPooled, user_id: i32) -> HttpResult<Option<(i32, String, Option<String>, Option<String>)>> {
    let user = users
      .select((id, username, skin, cape))
      .filter(id.eq(user_id))
      .first::<(i32, String, Option<String>, Option<String>)>(db)
      .optional()?;

    Ok(user)
  }

  #[allow(unused)]
  pub fn get_by_username(db: &mut DbPooled, user_name: String) -> HttpResult<Option<User>> {
    let user = users
      .select((id, username, skin, cape))
      .filter(username.eq(user_name))
      .first::<User>(db)
      .optional()?;

    Ok(user)
  }

  // Установка скина пользователю
  pub async fn set_skin(db: &mut DbPooled, user_name: String, new_skin: Option<String>) -> HttpResult<()> {
    let user_id = Self::ensure_user_exists(db, &user_name).await?;

    // Получаем текущий скин
    let current_skin: Option<String> = users
      .select(skin)
      .filter(id.eq(user_id))
      .first::<Option<String>>(db)
      .optional()?
      .flatten();

    // Убираем старый скин, если был
    if let Some(old_skin) = &current_skin {
      SkinCapeCacheRepository::remove(db, old_skin.clone())?;
    }

    // Добавляем новый скин, если есть
    if let Some(skin_hash) = &new_skin {
      SkinCapeCacheRepository::add(db, skin_hash.clone())?;
    }

    // Обновляем в БД
    diesel::update(users.filter(id.eq(user_id)))
      .set(skin.eq(new_skin))
      .execute(db)?;

    Ok(())
  }

  // Установка плаща пользователю
  pub async fn set_cape(db: &mut DbPooled, user_name: String, new_cape: Option<String>) -> HttpResult<()> {
    let user_id = Self::ensure_user_exists(db, &user_name).await?;

    // Получаем текущий плащ
    let current_cape: Option<String> = users
      .select(cape)
      .filter(id.eq(user_id))
      .first::<Option<String>>(db)
      .optional()?
      .flatten();

    // Убираем старый плащ, если был
    if let Some(old_cape) = &current_cape {
      SkinCapeCacheRepository::remove(db, old_cape.clone())?;
    }

    // Добавляем новый плащ, если есть
    if let Some(cape_hash) = &new_cape {
      SkinCapeCacheRepository::add(db, cape_hash.clone())?;
    }

    // Обновляем в БД
    diesel::update(users.filter(id.eq(user_id)))
      .set(cape.eq(new_cape))
      .execute(db)?;

    Ok(())
  }

  #[allow(unused)]
  pub async fn delete(db: &mut DbPooled, user_name: String) -> HttpResult<()> {
    let user_id = Self::ensure_user_exists(db, &user_name).await?;

    // Получаем скин и плащ перед удалением
    let (current_skin, current_cape): (Option<String>, Option<String>) = users
      .select((skin, cape))
      .filter(id.eq(user_id))
      .first::<(Option<String>, Option<String>)>(db)
      .optional()?
      .unwrap_or((None, None));

    if let Some(curr_skin) = current_skin {
      SkinCapeCacheRepository::remove(db, curr_skin)?;
    }
    if let Some(curr_cape) = current_cape {
      SkinCapeCacheRepository::remove(db, curr_cape)?;
    }

    // Удаляем пользователя
    diesel::delete(users.filter(id.eq(user_id)))
      .execute(db)?;

    Ok(())
  }
}
