use anyhow::Context;
use axum::{extract::{Multipart, Path}, http::HeaderMap, response::Response, routing::{get, post}, Json};
use dixxxie::{controller::Controller, response::{HttpError, HttpMessage, HttpResult}};
use reqwest::StatusCode;
use crate::{service::skincape::SkinCapeService, AppState};

pub struct SkinCapeController;

impl SkinCapeController {
  async fn get_skin(
    Path(username): Path<String>
  ) -> HttpResult<Response> {
    SkinCapeService::get_skin(username)
      .await
  }

  async fn get_cape(
    Path(username): Path<String>
  ) -> HttpResult<Response> {
    SkinCapeService::get_cape(username)
      .await
  }

  fn get_authorization(
    headers: &HeaderMap
  ) -> HttpResult<String> {
    let bearer = headers.get("authorization")
      .context(HttpError::new("Authorization Bearer не предоставлен", Some(StatusCode::UNAUTHORIZED)))?
      .to_str()?
      .to_string();

    if !bearer.starts_with("Bearer ") {
      return Err(HttpError::new("Authorization Bearer не предоставлен", Some(StatusCode::UNAUTHORIZED)))
    }

    let binding = bearer.split("Bearer ")
      .collect::<Vec<&str>>();

    let token = binding
      .get(1)
      .context(HttpError::new("Authorization Bearer не предоставлен", Some(StatusCode::UNAUTHORIZED)))?;

    Ok(token.to_string())
  }
  async fn update_skin(
    headers: HeaderMap,
    Path(username): Path<String>,
    multipart: Multipart,
  ) -> HttpResult<Json<HttpMessage>> {
    let token = Self::get_authorization(&headers)?;

    SkinCapeService::update_skin(token, username, multipart)
      .await
  }

  async fn update_cape(
    headers: HeaderMap,
    Path(username): Path<String>,
    multipart: Multipart,
  ) -> HttpResult<Json<HttpMessage>> {
    let token = Self::get_authorization(&headers)?;

    SkinCapeService::update_cape(token, username, multipart)
      .await
  }
}

impl Controller<AppState> for SkinCapeController {
  fn register(&self, router: axum::Router<AppState>) -> axum::Router<AppState> {
    router
      .route("/skin/{username}", get(Self::get_skin))
      .route("/cape/{username}", get(Self::get_cape))

      .route("/skin/{username}", post(Self::update_skin))
      .route("/cape/{username}", post(Self::update_cape))
  }
}