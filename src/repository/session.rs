use diesel::prelude::*;
use adjust::{database::{postgres::Postgres, Database}, response::{HttpError, NonJsonHttpResult}};
use crate::{models::{session::{Session, SessionAdd, SessionData}, users::GlobalUserData}, schema::sessions::{self, accesstoken}, service::random::RandomService};

use super::users::UsersRepository;

pub struct SessionRepository;

impl SessionRepository {
  pub fn update(
    db: &mut Database<Postgres>,
    user: GlobalUserData
  ) -> NonJsonHttpResult<Session> {
    let user_id = user.id;
    let username = user.username;
    let new_accesstoken = RandomService::generate_access_token();

    UsersRepository::ensure_user_exists(db, &username)?;

    let result = db.transaction::<Session, HttpError, _>(|db| {
      let existing_session = sessions::table
        .filter(sessions::username.eq(&username))
        .first::<Session>(db)
        .optional()?;

      match existing_session {
        Some(mut session) => {
          // обновляем токен для найденной сессии
          diesel::update(sessions::table.filter(sessions::id.eq(session.id)))
            .set(accesstoken.eq(&new_accesstoken))
            .execute(db)?;

          session.accesstoken = new_accesstoken;
          Ok(session)
        }
        None => {
          // создаем новую сессию
          let new_session = SessionAdd {
            user_id,
            username: username.clone(),
            uuid: RandomService::generate_uuid(username.clone())?,
            accesstoken: new_accesstoken.clone(),
            serverid: None,
          };

          // пихаем в бд
          let new_session = diesel::insert_into(sessions::table)
            .values(&new_session)
            .get_result::<Session>(db)?;

          Ok(new_session)
        }
      }
    })?;

    Ok(result)
  }

  pub fn update_serverid(
    db: &mut Database<Postgres>,
    data: SessionData
  ) -> NonJsonHttpResult<usize> {
    let result = diesel::update(
        sessions::table
        .filter(
          sessions::uuid.eq(data.uuid)
            .and(sessions::accesstoken.eq(data.accesstoken))
      )
    ).set(sessions::serverid.eq(data.serverid))
      .execute(db)?;

    Ok(result)
  }

  pub fn find_by_username(
    db: &mut Database<Postgres>,
    username: String
  ) -> NonJsonHttpResult<Session> {
    Ok(sessions::table
      .filter(sessions::username.eq(username))
      .first::<Session>(db)?)
  }

  pub fn find_by_uuid(
    db: &mut Database<Postgres>,
    uuid: String
  ) -> NonJsonHttpResult<Session> {
    Ok(sessions::table
      .filter(sessions::uuid.eq(uuid))
      .first::<Session>(db)?)
  }
}