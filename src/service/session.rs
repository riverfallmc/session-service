#![allow(dead_code)]

use anyhow::anyhow;
use base64::Engine;
use dixxxie::{connection::DbPooled, response::{HttpError, HttpResult}};
use once_cell::sync::Lazy;
use reqwest::StatusCode;
use crate::{models::session::{PlayerJoinResponse, PlayerJoinResponseProperty, PlayerJoinResponsePropertyValue, PlayerJoinResponseTextures, SessionData, TextureData}, repository::session::SessionRepository, service::auth::AuthService};
use super::{signer::SignerService, time::TimeService};

static BASE_URL: Lazy<String> = Lazy::new(|| {
  if cfg!(debug_assertions) {String::from("https://localhost")} else {String::from("https://riverfall.ru")}
});

pub struct SessionService;

impl SessionService {
  fn get_player_textures(
    id: String,
    name: String
  ) -> HttpResult<PlayerJoinResponsePropertyValue> {
    let textures = PlayerJoinResponsePropertyValue {
      timestamp: TimeService::get_timestamp()?,
      profile_id: id,
      profile_name: name.clone(),
      textures: PlayerJoinResponseTextures {
        skin: TextureData {
          url: format!("{}/api/session/skin/{name}.png", *BASE_URL)
        },
        cape: Some(TextureData {
          url: format!("{}/api/session/cape/{name}.png", *BASE_URL)
        })
      },
    };

    Ok(textures)
  }

  fn collect_join_response(
    id: String,
    name: String
  ) -> HttpResult<PlayerJoinResponse> {
    let textures = Self::get_player_textures(id.clone(), name.clone())?;
    let value = serde_json::to_string(&textures)?;
    let value_encoded = base64::engine::general_purpose::STANDARD
      .encode(value);

    let properties = PlayerJoinResponseProperty {
      name: String::from("textures"),
      signature: SignerService::sign(value_encoded.clone())?,
      value: value_encoded
    };

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
      .await?;

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

  pub fn has_joined(
    db: &mut DbPooled,
    username: String,
    server_id: String
  ) -> HttpResult<PlayerJoinResponse> {
    let session = SessionRepository::find_by_username(db, username.clone())
      .map_err(|_| HttpError::new("Сессия не была найдена", Some(StatusCode::UNAUTHORIZED)))?;

    if session.serverid.unwrap_or_default() != server_id {
      return Err(HttpError::new("Сессия не была найдена", Some(StatusCode::UNAUTHORIZED)));
    }

    Self::collect_join_response(session.uuid, username)
  }
}