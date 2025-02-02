use std::path::PathBuf;
use axum::{extract::Multipart, http::StatusCode, response::Response, Json};
use dixxxie::response::{HttpError, HttpMessage, HttpResult};
use once_cell::sync::Lazy;
use crate::service::auth::AuthService;
use super::{fs::FileSystemService, multipart::MultipartService};

pub static SKINS_BASEDIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("/app/data/skins"));
pub static CAPES_BASEDIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("/app/data/capes"));

pub struct SkinCapeService;

impl SkinCapeService {
  fn format_username(username: &str) -> String {
    if let Some(dot_index) = username.rfind('.') {
      format!("{}.png", &username[..dot_index])
    } else {
      format!("{username}.png")
    }
  }

  fn find_skin(
    username: String
  ) -> Option<PathBuf> {
    let path = SKINS_BASEDIR.join(Self::format_username(&username));

    if !path.exists() {
      return None;
    }

    Some(path)
  }

  fn find_cape(
    username: String
  ) -> Option<PathBuf> {
    let path = CAPES_BASEDIR.join(Self::format_username(&username));

    if !path.exists() {
      return None
    }

    Some(path)
  }

  pub async fn get_skin(
    username: String
  ) -> HttpResult<Response> {
    let path = Self::find_skin(username)
      .ok_or_else(|| HttpError::new("Скин не был найден", Some(StatusCode::NOT_FOUND)))?;

    FileSystemService::read_file(path)
      .await
  }

  pub async fn get_cape(
    username: String
  ) -> HttpResult<Response> {
    let path = Self::find_cape(username)
      .ok_or_else(|| HttpError::new("Плащ не был найден", Some(StatusCode::NOT_FOUND)))?;

    FileSystemService::read_file(path)
      .await
  }

  async fn save_file(
    token: String,
    username: String,
    multipart: Multipart,
    success: &str,
    save_in: &str
  ) -> HttpResult<Json<HttpMessage>> {
    AuthService::check_token_owner(token, &username)
      .await?;

    let content = MultipartService::read(multipart)
      .await?;

    FileSystemService::save(save_in, format!("{username}.png"), content)
      .await
      .map_err(|_| HttpError::new("Не получилось сохранить", None))?;

    Ok(Json(HttpMessage::new(success)))
  }

  pub async fn update_skin(
    token: String,
    username: String,
    skin: Multipart
  ) -> HttpResult<Json<HttpMessage>> {
    Self::save_file(token, username, skin, "Скин был успешно применён", "skins")
      .await
  }

  pub async fn update_cape(
    token: String,
    username: String,
    cape: Multipart
  ) -> HttpResult<Json<HttpMessage>> {
    Self::save_file(token, username, cape, "Плащ был успешно применён", "capes")
    .await
  }
}