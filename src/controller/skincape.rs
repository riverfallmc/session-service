use axum::{extract::{Multipart, Path, Query, State}, response::Response, routing::{get, post}, Json};
use adjust::{controller::Controller, response::{HttpMessage, HttpResult, NonJsonHttpResult}};
use crate::{models::users::GlobalUserData, service::skincape::SkinCapeService, AppState};

pub struct SkinCapeController;

impl SkinCapeController {
  async fn get_skin(
    Path(img): Path<String>
  ) -> NonJsonHttpResult<Response> {
    SkinCapeService::get_skin(img)
      .await
  }

  async fn get_cape(
    Path(img): Path<String>
  ) -> NonJsonHttpResult<Response> {
    SkinCapeService::get_cape(img)
      .await
  }

  async fn update_skin(
    State(state): State<AppState>,
    Query(user): Query<GlobalUserData>,
    multipart: Multipart,
  ) -> HttpResult<HttpMessage> {
    let mut db = state.postgres.get()?;

    Ok(Json(SkinCapeService::update_skin(&mut db, user, multipart)
      .await?))
  }

  async fn update_cape(
    State(state): State<AppState>,
    Query(user): Query<GlobalUserData>,
    multipart: Multipart,
  ) -> HttpResult<HttpMessage> {
    let mut db = state.postgres.get()?;

    Ok(Json(SkinCapeService::update_cape(&mut db, user, multipart)
      .await?))
  }
}

impl Controller<AppState> for SkinCapeController {
  fn new() -> anyhow::Result<Box<Self>> {
    Ok(Box::new(Self))
  }

  fn register(&self, router: axum::Router<AppState>) -> axum::Router<AppState> {
    #[allow(unused)]
    SkinCapeService::valid_folders()
      .expect("unable to create dirs");

    router
      .route("/skin/{img}", get(Self::get_skin))
      .route("/cape/{img}", get(Self::get_cape))

      .route("/skin", post(Self::update_skin))
      .route("/cape", post(Self::update_cape))
  }
}