#![allow(dead_code)]

use anyhow::anyhow;
use base64::Engine;
use dixxxie::{connection::DbPooled, response::{HttpError, HttpResult}};
use reqwest::StatusCode;
use crate::{models::session::{PlayerJoinResponse, PlayerJoinResponseProperty, PlayerJoinResponsePropertyValue, PlayerJoinResponseTextures, SessionData, TextureData}, repository::session::SessionRepository, service::auth::AuthService};
use super::{signer::SignerService, skincape::SkinCapeService, time::TimeService};

pub struct SessionService;

impl SessionService {
  async fn get_player_textures(
    id: String,
    name: String
  ) -> HttpResult<PlayerJoinResponsePropertyValue> {
    let mut textures = PlayerJoinResponseTextures::default();

    if let Ok(url) = SkinCapeService::get_skin_url(&name) {
      textures.skin = Some(TextureData {
        url
      })
    }

    if let Ok(url) = SkinCapeService::get_cape_url(&name) {
      textures.cape = Some(TextureData {
        url
      })
    }

    let textures = PlayerJoinResponsePropertyValue {
      timestamp: TimeService::get_timestamp()?,
      profile_id: id,
      profile_name: name.clone(),
      textures,
    };

    Ok(textures)
  }

  async fn user_profile(
    id: String,
    name: String,
    unsigned: bool
  ) -> HttpResult<PlayerJoinResponse> {
    let textures = Self::get_player_textures(id.clone(), name.clone())
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
    db: &mut DbPooled,
    token: String
  ) -> HttpResult<SessionData> {
    let user = AuthService::get_by_token(token)
      .await
      .map_err(|e| anyhow!("Ошибка сервера авторизации: {e}"))?;

    let data = SessionRepository::update(db, user)
      .map_err(|e| anyhow!("Неизвестная ошибка: {e}"))?;

    Ok(data.into())
  }

  pub fn join(
    db: &mut DbPooled,
    data: SessionData
  ) -> HttpResult<usize> {
    SessionRepository::update_serverid(db, data)
  }

  pub async fn has_joined(
    db: &mut DbPooled,
    username: String,
    server_id: String
  ) -> HttpResult<PlayerJoinResponse> {
    let session = SessionRepository::find_by_username(db, username.clone())
      .map_err(|_| HttpError::new("Сессия не была найдена", Some(StatusCode::UNAUTHORIZED)))?;

    if session.serverid.unwrap_or_default() != server_id {
      return Err(HttpError::new("Сессия не была найдена", Some(StatusCode::UNAUTHORIZED)));
    }

    Self::user_profile(session.uuid, username, false)
      .await
  }

  pub async fn get_profile(
    db: &mut DbPooled,
    uuid: String,
    unsigned: bool
  ) -> HttpResult<PlayerJoinResponse> {
    let session = SessionRepository::find_by_uuid(db, uuid.clone())
      .map_err(|_| HttpError::new("Сессия не была найдена", Some(StatusCode::UNAUTHORIZED)))?;

    Self::user_profile(session.uuid, session.username, unsigned)
      .await
  }
}