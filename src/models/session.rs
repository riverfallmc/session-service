use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::sessions;

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Session {
  pub id: i32,
  pub user_id: i32,
  pub username: String,
  pub uuid: String,
  pub accesstoken: String,
  pub serverid: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct SessionData {
  /// uuid
  #[serde(rename="selectedProfile")]
  pub uuid: String,
  #[serde(rename="accessToken")]
  pub accesstoken: String,
  #[serde(rename="serverId")]
  pub serverid: String,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerJoinResponse {
  pub id: String,
  pub name: String,
  pub properties: Vec<PlayerJoinResponseProperty>
}

#[derive(Serialize, Deserialize)]
pub(crate) struct PlayerJoinResponseProperty {
  pub name: String,
  /// Строка, закодированная BASE64,\
  /// внутри которой лежит PlayerJoinResponsePropertyValue
  pub value: String,
  /// Value - BASE64
  /// Signature - Подписанный приватным ключём Value
  #[serde(skip_serializing_if = "Option::is_none")]
  pub signature: Option<String>
}

#[derive(Serialize, Deserialize)]
pub(crate) struct PlayerJoinResponsePropertyValue {
  pub timestamp: u64,
  #[serde(rename="profileId")]
  pub profile_id: String,
  #[serde(rename="profileName")]
  pub profile_name: String,
  pub textures: PlayerJoinResponseTextures
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct PlayerJoinResponseTextures {
  /// URL на скин игрока
  #[serde(rename="SKIN")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub skin: Option<TextureData>,
  /// URL на плащ игрока
  #[serde(rename="CAPE")]
  #[serde(skip_serializing_if = "Option::is_none")]
  pub cape: Option<TextureData>
}

#[derive(Serialize, Deserialize)]
pub(crate) struct TextureData {
  pub url: String
}

impl From<Session> for SessionData {
  fn from(value: Session) -> Self {
    SessionData {
      uuid: value.uuid,
      accesstoken: value.accesstoken,
      serverid: value.serverid.unwrap_or_default() // не оч безопасно
    }
  }
}