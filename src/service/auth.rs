use std::env;

use adjust::response::{HttpError, HttpResult};
use axum::Json;
use once_cell::sync::Lazy;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use crate::controller::session::JWTBody;

static CLIENT: Lazy<Client> = Lazy::new(Client::new);
static AUTH_SERVICE_URL: Lazy<String> = Lazy::new(|| env::var("AUTH_SERVICE_URL").expect("No AUTH_SERVICE_URL"));

#[derive(Serialize, Deserialize)]
pub struct UserData {
  pub id: i32,
  pub username: String
}

pub struct AuthService;

impl AuthService {
  pub async fn get_by_token(
    token: String
  ) -> HttpResult<UserData> {
    let result = CLIENT.post(format!("http://{}/owner", *AUTH_SERVICE_URL))
      .json(&JWTBody {
        token
      })
      .send()
      .await?;

    Ok(Json(result.json().await?))
  }

  pub async fn get_token_owner(token: String) -> HttpResult<String> {
    let result = Self::get_by_token(token)
      .await
      .map_err(|_| HttpError::new("Невалидный токен", Some(StatusCode::UNAUTHORIZED)))?;

    Ok(Json(result.0.username))
  }
}