use adjust::database::postgres::Postgres;
use adjust::database::Database;
use adjust::response::NonJsonHttpResult;
use diesel::prelude::*;
use crate::models::users::User;
use crate::schema::users::dsl::*;
use crate::repository::skincapecache::SkinCapeCacheRepository;

pub struct UsersRepository;

impl UsersRepository {
  pub fn ensure_user_exists(db: &mut Database<Postgres>, user_name: &str) -> NonJsonHttpResult<i32> {
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
  pub async fn get_by_id(db: &mut Database<Postgres>, user_id: i32) -> NonJsonHttpResult<Option<(i32, String, Option<String>, Option<String>)>> {
    let user = users
      .select((id, username, skin, cape))
      .filter(id.eq(user_id))
      .first::<(i32, String, Option<String>, Option<String>)>(db)
      .optional()?;

    Ok(user)
  }

  #[allow(unused)]
  pub fn get_by_username(db: &mut Database<Postgres>, user_name: String) -> NonJsonHttpResult<Option<User>> {
    let user = users
      .select((id, username, skin, cape))
      .filter(username.eq(user_name))
      .first::<User>(db)
      .optional()?;

    Ok(user)
  }

  // Установка скина пользователю
  pub async fn set_skin(db: &mut Database<Postgres>, user_name: String, new_skin: Option<String>) -> NonJsonHttpResult<()> {
    let user_id = Self::ensure_user_exists(db, &user_name)?;

    // Получаем текущий скин
    let current_skin: Option<String> = users
      .select(skin)
      .filter(id.eq(user_id))
      .first::<Option<String>>(db)
      .optional()?
      .flatten();

    if let Some(skin_hash) = &new_skin {
      #[allow(unused)]
      SkinCapeCacheRepository::add(db, skin_hash.clone())?;
    }

    if let Some(old_skin) = &current_skin {
      #[allow(unused)]
      SkinCapeCacheRepository::remove(db, old_skin.clone())
        .await?;
    }

    // Обновляем в БД
    diesel::update(users.filter(id.eq(user_id)))
      .set(skin.eq(new_skin))
      .execute(db)?;

    Ok(())
  }

  pub async fn set_cape(db: &mut Database<Postgres>, user_name: String, new_cape: Option<String>) -> NonJsonHttpResult<()> {
    let user_id = Self::ensure_user_exists(db, &user_name)?;

    // Получаем текущий плащ
    let current_cape: Option<String> = users
      .select(cape)
      .filter(id.eq(user_id))
      .first::<Option<String>>(db)
      .optional()?
      .flatten();

    if let Some(old_cape) = &current_cape {
      #[allow(unused)]
      SkinCapeCacheRepository::remove(db, old_cape.clone())
        .await?;
    }

    if let Some(cape_hash) = &new_cape {
      #[allow(unused)]
      SkinCapeCacheRepository::add(db, cape_hash.clone())?;
    }

    // Обновляем в БД
    diesel::update(users.filter(id.eq(user_id)))
      .set(cape.eq(new_cape))
      .execute(db)?;

    Ok(())
  }

  #[allow(unused)]
  pub async fn delete(db: &mut Database<Postgres>, user_name: String) -> NonJsonHttpResult<()> {
    let user_id = Self::ensure_user_exists(db, &user_name)?;

    // Получаем скин и плащ перед удалением
    let (current_skin, current_cape): (Option<String>, Option<String>) = users
      .select((skin, cape))
      .filter(id.eq(user_id))
      .first::<(Option<String>, Option<String>)>(db)
      .optional()?
      .unwrap_or((None, None));

    if let Some(curr_skin) = current_skin {
      SkinCapeCacheRepository::remove(db, curr_skin)
        .await?;
    }
    if let Some(curr_cape) = current_cape {
      SkinCapeCacheRepository::remove(db, curr_cape)
        .await?;
    }

    // Удаляем пользователя
    diesel::delete(users.filter(id.eq(user_id)))
      .execute(db)?;

    Ok(())
  }
}
