use std::{env, fs, path::PathBuf};
use anyhow::anyhow;
use axum::{extract::Multipart, http::StatusCode, response::Response};
use adjust::{database::{postgres::Postgres, Database}, response::{HttpError, HttpMessage, NonJsonHttpResult}};
use once_cell::sync::Lazy;
use crate::{models::users::{GlobalUserData, User}, repository::users::UsersRepository};
use super::{fs::FileSystemService, hasher::HasherService, multipart::MultipartService};

pub static SKINCAPES_BASEDIR: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("/app/data/skincapes"));
static BASE_URL: Lazy<String> = Lazy::new(|| {
  env::var("OVERRIDE_LINK")
    .unwrap_or("https://riverfall.ru".to_string())
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
    let path = SKINCAPES_BASEDIR.join(Self::format_img(img));

    if !path.exists() {
      return None;
    }

    Some(path)
  }

  pub fn find_cape(
    img: &str
  ) -> Option<PathBuf> {
    let path = SKINCAPES_BASEDIR.join(Self::format_img(img));

    if !path.exists() {
      return None
    }

    Some(path)
  }

  #[allow(unused)]
  pub fn get_skin_url(
    db: &mut Database<Postgres>,
    username: &str
  ) -> NonJsonHttpResult<String> {
    let user = UsersRepository::get_by_username(db, username.to_string())?;

    if user.is_none() {
      return Err(HttpError::new("Игрок не был найден", Some(StatusCode::NOT_FOUND)));
    }

    Self::check_skin_url(&user.unwrap())
  }

  pub fn check_skin_url(
    user: &User
  ) -> NonJsonHttpResult<String> {
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
    db: &mut Database<Postgres>,
    username: &str
  ) -> NonJsonHttpResult<String> {
    let user = UsersRepository::get_by_username(db, username.to_string())?;

    if user.is_none() {
      return Err(HttpError::new("Игрок не был найден", Some(StatusCode::NOT_FOUND)));
    }

    Self::check_cape_url(&user.unwrap())
  }

  pub fn check_cape_url(
    user: &User
  ) -> NonJsonHttpResult<String> {
    let img = user.cape
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
  ) -> NonJsonHttpResult<Response> {
    let path = Self::find_skin(&img)
      .ok_or_else(|| HttpError::new("Скин не был найден", Some(StatusCode::NOT_FOUND)))?;

    FileSystemService::read_file(path)
      .await
  }

  pub async fn get_cape(
    img: String
  ) -> NonJsonHttpResult<Response> {
    let path = Self::find_cape(&img)
      .ok_or_else(|| HttpError::new("Плащ не был найден", Some(StatusCode::NOT_FOUND)))?;

    FileSystemService::read_file(path)
      .await
  }

  async fn save_file(
    db: &mut Database<Postgres>,
    username: String,
    multipart: Multipart,
    success_text: &str,
    r#type: FileSaveType
  ) -> NonJsonHttpResult<HttpMessage> {
    let content = MultipartService::read(multipart)
      .await?;

    let hashed = HasherService::sha1_bytes(&content);
    let img = format!("{hashed}.png");

    #[allow(unused)]
    FileSystemService::save(img.clone(), content)
      .await
      .map_err(|e| HttpError(anyhow!("Не получилось сохранить: {e}"), None))?;

    #[allow(unused)]
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

    Ok(HttpMessage::new(success_text))
  }

  pub async fn update_skin(
    db: &mut Database<Postgres>,
    user: GlobalUserData,
    skin: Multipart
  ) -> NonJsonHttpResult<HttpMessage> {
    Self::save_file(db, user.username, skin, "Скин был успешно применён", FileSaveType::Skin)
      .await
  }

  pub async fn update_cape(
    db: &mut Database<Postgres>,
    user: GlobalUserData,
    cape: Multipart
  ) -> NonJsonHttpResult<HttpMessage> {
    Self::save_file(db, user.username, cape, "Плащ был успешно применён", FileSaveType::Cape)
      .await
  }

  pub fn valid_folders() -> NonJsonHttpResult<()> {
    let _ = fs::create_dir(SKINCAPES_BASEDIR.as_path());

    Ok(())
  }
}