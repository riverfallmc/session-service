use diesel::prelude::*;
use dixxxie::{connection::DbPooled, response::{HttpError, HttpResult}};
use crate::{models::session::{Session, SessionData}, schema::sessions::{self, accesstoken}, service::{auth::UserData, random::RandomService}};

pub struct SessionRepository;

impl SessionRepository {
  pub fn update(
    db: &mut DbPooled,
    user: UserData
  ) -> HttpResult<Session> {
    let user_id = user.id;
    let username = user.username;
    let new_accesstoken = RandomService::generate_access(username.clone())?;

    db.transaction::<Session, HttpError, _>(|db| {
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
          let new_session = Session {
            id: 0,
            user_id,
            username: username.clone(),
            uuid: RandomService::generate_uuid(username.clone()),
            accesstoken: new_accesstoken.clone(),
            serverid: None,
          };

          // пихаем в бд
          diesel::insert_into(sessions::table)
            .values(&new_session)
            .execute(db)?;

          Ok(new_session)
        }
      }
    })
  }

  pub fn update_serverid(
    db: &mut DbPooled,
    data: SessionData
  ) -> HttpResult<usize> {
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
    db: &mut DbPooled,
    username: String
  ) -> HttpResult<Session> {
    Ok(sessions::table
      .filter(sessions::username.eq(username))
      .first::<Session>(db)?)
  }
}