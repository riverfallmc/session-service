use anyhow::{anyhow, Context};
use base64::Engine;
use adjust::{database::{postgres::Postgres, Database}, response::{HttpError, NonJsonHttpResult}};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use crate::{models::{session::{PlayerJoinResponse, PlayerJoinResponseProperty, PlayerJoinResponsePropertyValue, PlayerJoinResponseTextures, SessionData, TextureData}, users::GlobalUserData}, repository::{session::SessionRepository, users::UsersRepository}};
use super::{signer::SignerService, skincape::SkinCapeService, time::TimeService};

#[derive(Deserialize, Serialize)]
pub struct PlayerProfile {
  skin: Option<String>,
  cape: Option<String>
}

pub struct SessionService;

impl SessionService {
  async fn get_player_textures(
    db: &mut Database<Postgres>,
    id: String,
    name: String
  ) -> NonJsonHttpResult<PlayerJoinResponsePropertyValue> {
    let textures = Self::profile(db, &name)?;

    let textures = PlayerJoinResponsePropertyValue {
      timestamp: TimeService::get_timestamp()?,
      profile_id: id,
      profile_name: name.clone(),
      textures,
    };

    Ok(textures)
  }

  pub fn profile(
    db: &mut Database<Postgres>,
    username: &str
  ) -> NonJsonHttpResult<PlayerJoinResponseTextures> {
    let mut textures = PlayerJoinResponseTextures::default();

    let user = UsersRepository::get_by_username(db, username.to_string())?
      .context(anyhow!("Игрок не был найден"))?;

    if let Ok(url) = SkinCapeService::check_skin_url(&user) {
      textures.skin = Some(TextureData {
        url
      })
    }

    if let Ok(url) = SkinCapeService::check_cape_url(&user) {
      textures.cape = Some(TextureData {
        url
      })
    }

    Ok(textures)
  }

  async fn user_profile(
    db: &mut Database<Postgres>,
    id: String,
    name: String,
    unsigned: bool
  ) -> NonJsonHttpResult<PlayerJoinResponse> {
    let textures = Self::get_player_textures(db, id.clone(), name.clone())
      .await?;

    let value = base64::engine::general_purpose::STANDARD
        .encode(serde_json::to_string(&textures)?);

    let mut properties = PlayerJoinResponseProperty {
      name: String::from("textures"),
      value: value.clone(),
      signature: Some(String::new())
    };

    if !unsigned {
      properties.signature = Some(SignerService::sign(&value)?);
    }

    Ok(PlayerJoinResponse {
      id,
      name,
      properties: vec![properties]
    })
  }

  pub async fn login(
    db: &mut Database<Postgres>,
    user: GlobalUserData
  ) -> NonJsonHttpResult<SessionData> {
    let data = SessionRepository::update(db, user)
      .map_err(|e| anyhow!("Неизвестная ошибка: {e}"))?;

    Ok(data.into())
  }

  pub fn join(
    db: &mut Database<Postgres>,
    data: SessionData
  ) -> NonJsonHttpResult<usize> {
    SessionRepository::update_serverid(db, data)
  }

  pub async fn has_joined(
    db: &mut Database<Postgres>,
    username: String,
    server_id: String
  ) -> NonJsonHttpResult<PlayerJoinResponse> {
    let session = SessionRepository::find_by_username(db, username.clone())
      .map_err(|_| HttpError::new("Сессия не была найдена", Some(StatusCode::UNAUTHORIZED)))?;

    if session.serverid.unwrap_or_default() != server_id {
      return Err(HttpError::new("Сессия не была найдена", Some(StatusCode::UNAUTHORIZED)));
    }

    Self::user_profile(db, session.uuid, username, false)
      .await
  }

  pub async fn get_profile(
    db: &mut Database<Postgres>,
    uuid: String,
    unsigned: bool
  ) -> NonJsonHttpResult<PlayerJoinResponse> {
    let session = SessionRepository::find_by_uuid(db, uuid.clone())
      .map_err(|_| HttpError::new("Сессия не была найдена", Some(StatusCode::UNAUTHORIZED)))?;

    Self::user_profile(db, session.uuid, session.username, unsigned)
      .await
  }
}