use anyhow::Context;
use axum::{extract::{Multipart, Path, State}, http::HeaderMap, response::Response, routing::{get, post}, Json};
use dixxxie::{controller::Controller, response::{HttpError, HttpMessage, HttpResult}};
use reqwest::StatusCode;
use crate::{service::skincape::SkinCapeService, AppState};

pub struct SkinCapeController;

impl SkinCapeController {
  async fn get_skin(
    Path(img): Path<String>
  ) -> HttpResult<Response> {
    SkinCapeService::get_skin(img)
      .await
  }

  async fn get_cape(
    Path(img): Path<String>
  ) -> HttpResult<Response> {
    SkinCapeService::get_cape(img)
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
    State(state): State<AppState>,
    multipart: Multipart,
  ) -> HttpResult<Json<HttpMessage>> {
    let token = Self::get_authorization(&headers)?;
    let mut db = state.postgres.get()?;

    SkinCapeService::update_skin(&mut db, token, multipart)
      .await
  }

  async fn update_cape(
    headers: HeaderMap,
    State(state): State<AppState>,
    multipart: Multipart,
  ) -> HttpResult<Json<HttpMessage>> {
    let token = Self::get_authorization(&headers)?;
    let mut db = state.postgres.get()?;

    SkinCapeService::update_cape(&mut db, token, multipart)
      .await
  }
}

impl Controller<AppState> for SkinCapeController {
  fn register(&self, router: axum::Router<AppState>) -> axum::Router<AppState> {
    SkinCapeService::valid_folders()
      .expect("unable to create dirs");

    router
      .route("/skin/{img}", get(Self::get_skin))
      .route("/cape/{img}", get(Self::get_cape))

      .route("/skin", post(Self::update_skin))
      .route("/cape", post(Self::update_cape))
  }
}