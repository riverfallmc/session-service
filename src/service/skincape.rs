use std::{fs, path::PathBuf};
use anyhow::anyhow;
use axum::{extract::Multipart, http::StatusCode, response::Response, Json};
use dixxxie::response::{HttpError, HttpMessage, HttpResult};
use once_cell::sync::Lazy;
use crate::service::auth::AuthService;
use super::{fs::FileSystemService, multipart::MultipartService};

pub static SKINS_BASEDIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("/app/data/skins"));
pub static CAPES_BASEDIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("/app/data/capes"));

static BASE_URL: Lazy<String> = Lazy::new(|| {
  // TODO @ вернуть ссылки
  #[allow(clippy::if_same_then_else)]
  if cfg!(debug_assertions) {String::from("https://localhost")} else {String::from("https://localhost")}
});

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
    username: &str
  ) -> Option<PathBuf> {
    let path = SKINS_BASEDIR.join(Self::format_username(username));

    if !path.exists() {
      return None;
    }

    Some(path)
  }

  fn find_cape(
    username: &str
  ) -> Option<PathBuf> {
    let path = CAPES_BASEDIR.join(Self::format_username(username));

    if !path.exists() {
      return None
    }

    Some(path)
  }

  pub fn get_skin_url(
    username: &str
  ) -> HttpResult<String> {
    Self::find_skin(username)
      .ok_or_else(|| HttpError::new("Скин не был найден", Some(StatusCode::NOT_FOUND)))?;

    Ok(format!("{}/api/session/skin/{username}.png", *BASE_URL))
  }

  pub fn get_cape_url(
    username: &str
  ) -> HttpResult<String> {
    Self::find_cape(username)
      .ok_or_else(|| HttpError::new("Плащ не был найден", Some(StatusCode::NOT_FOUND)))?;

    Ok(format!("{}/api/session/cape/{username}.png", *BASE_URL))
  }

  pub async fn get_skin(
    username: String
  ) -> HttpResult<Response> {
    let path = Self::find_skin(&username)
      .ok_or_else(|| HttpError::new("Скин не был найден", Some(StatusCode::NOT_FOUND)))?;

    FileSystemService::read_file(path)
      .await
  }

  pub async fn get_cape(
    username: String
  ) -> HttpResult<Response> {
    let path = Self::find_cape(&username)
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
      .map_err(|e| HttpError(anyhow!("Не получилось сохранить: {e}"), None))?;

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

  pub fn valid_folders() -> HttpResult<()> {
    let _ = fs::create_dir(SKINS_BASEDIR.as_path());
    let _ = fs::create_dir(CAPES_BASEDIR.as_path());

    Ok(())
  }
}