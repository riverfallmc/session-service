use std::{fs, path::PathBuf};
use anyhow::anyhow;
use axum::{extract::Multipart, http::StatusCode, response::Response, Json};
use dixxxie::{connection::DbPooled, response::{HttpError, HttpMessage, HttpResult}};
use once_cell::sync::Lazy;
use crate::{models::users::User, repository::users::UsersRepository, service::auth::AuthService};
use super::{fs::FileSystemService, hasher::HasherService, multipart::MultipartService};

pub static SKINS_BASEDIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("/app/data/skins"));
pub static CAPES_BASEDIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("/app/data/capes"));

static BASE_URL: Lazy<String> = Lazy::new(|| {
  // TODO @ вернуть ссылки
  #[allow(clippy::if_same_then_else)]
  if cfg!(debug_assertions) {String::from("https://localhost")} else {String::from("https://localhost")}
});

enum FileSaveType {
  Skin,
  Cape
}

pub struct SkinCapeService;

impl SkinCapeService {
  fn format_img(img: &str) -> String {
    if let Some(dot_index) = img.rfind('.') {
      format!("{}.png", &img[..dot_index])
    } else {
      format!("{img}.png")
    }
  }

  pub fn find_skin(
    img: &str
  ) -> Option<PathBuf> {
    let path = SKINS_BASEDIR.join(Self::format_img(img));

    if !path.exists() {
      return None;
    }

    Some(path)
  }

  pub fn find_cape(
    img: &str
  ) -> Option<PathBuf> {
    let path = CAPES_BASEDIR.join(Self::format_img(img));

    if !path.exists() {
      return None
    }

    Some(path)
  }

  #[allow(unused)]
  pub fn get_skin_url(
    db: &mut DbPooled,
    username: &str
  ) -> HttpResult<String> {
    let user = UsersRepository::get_by_username(db, username.to_string())?;

    if user.is_none() {
      return Err(HttpError::new("Игрок не был найден", Some(StatusCode::NOT_FOUND)));
    }

    Self::check_skin_url(&user.unwrap())
  }

  pub fn check_skin_url(
    user: &User
  ) -> HttpResult<String> {
    let img = user.skin
      .clone();

    if let Some(skin) = img {
      Self::find_skin(&skin)
        .ok_or_else(|| HttpError::new("Скин не был найден", Some(StatusCode::NOT_FOUND)))?;

      return Ok(format!("{}/api/session/skin/{skin}", *BASE_URL));
    }

     Err(HttpError::new("Скин не был найден", Some(StatusCode::NOT_FOUND)))
  }

  #[allow(unused)]
  pub fn get_cape_url(
    db: &mut DbPooled,
    username: &str
  ) -> HttpResult<String> {
    let user = UsersRepository::get_by_username(db, username.to_string())?;

    if user.is_none() {
      return Err(HttpError::new("Игрок не был найден", Some(StatusCode::NOT_FOUND)));
    }

    Self::check_cape_url(&user.unwrap())
  }

  pub fn check_cape_url(
    user: &User
  ) -> HttpResult<String> {
    let img = user.skin
      .clone();

    if let Some(cape) = img {
      Self::find_cape(&cape)
        .ok_or_else(|| HttpError::new("Плащ не был найден", Some(StatusCode::NOT_FOUND)))?;

      return Ok(format!("{}/api/session/cape/{cape}", *BASE_URL));
    }

     Err(HttpError::new("Плащ не был найден", Some(StatusCode::NOT_FOUND)))
  }

  pub async fn get_skin(
    img: String
  ) -> HttpResult<Response> {
    let path = Self::find_skin(&img)
      .ok_or_else(|| HttpError::new("Скин не был найден", Some(StatusCode::NOT_FOUND)))?;

    FileSystemService::read_file(path)
      .await
  }

  pub async fn get_cape(
    img: String
  ) -> HttpResult<Response> {
    let path = Self::find_cape(&img)
      .ok_or_else(|| HttpError::new("Плащ не был найден", Some(StatusCode::NOT_FOUND)))?;

    FileSystemService::read_file(path)
      .await
  }

  async fn save_file(
    db: &mut DbPooled,
    token: String,
    multipart: Multipart,
    success_text: &str,
    r#type: FileSaveType
  ) -> HttpResult<Json<HttpMessage>> {
    let username = AuthService::get_token_owner(token)
      .await?;

    let content = MultipartService::read(multipart)
      .await?;

    let save_in = match r#type {
      FileSaveType::Skin => "skins",
      FileSaveType::Cape => "capes"
    };

    let hashed = HasherService::sha1_bytes(&content);
    let img = format!("{hashed}.png");

    FileSystemService::save(save_in, img.clone(), content)
      .await
      .map_err(|e| HttpError(anyhow!("Не получилось сохранить: {e}"), None))?;

    match r#type {
      FileSaveType::Skin => {
        UsersRepository::set_skin(db, username, Some(img))
          .await?;
      },
      FileSaveType::Cape => {
        UsersRepository::set_cape(db, username, Some(img))
          .await?;
      }
    }

    Ok(Json(HttpMessage::new(success_text)))
  }

  pub async fn update_skin(
    db: &mut DbPooled,
    token: String,
    skin: Multipart
  ) -> HttpResult<Json<HttpMessage>> {
    Self::save_file(db, token, skin, "Скин был успешно применён", FileSaveType::Skin)
      .await
  }

  pub async fn update_cape(
    db: &mut DbPooled,
    token: String,
    cape: Multipart
  ) -> HttpResult<Json<HttpMessage>> {
    Self::save_file(db, token, cape, "Плащ был успешно применён", FileSaveType::Cape)
    .await
  }

  pub fn valid_folders() -> HttpResult<()> {
    let _ = fs::create_dir(SKINS_BASEDIR.as_path());
    let _ = fs::create_dir(CAPES_BASEDIR.as_path());

    Ok(())
  }
}