use axum::{extract::{Query, State}, routing::{get, post}, Json};
use dixxxie::{controller::Controller, response::HttpResult};
use serde::{Deserialize, Serialize};
use crate::{models::session::{PlayerJoinResponse, SessionData}, service::session::SessionService, AppState};

#[derive(Serialize, Deserialize)]
pub struct JWTBody {
  pub token: String
}

#[derive(Serialize, Deserialize)]
struct HasJoinedQuery {
  pub username: String,
  #[serde(rename="serverId")]
  pub server_id: String,
}

pub struct SessionController;

impl SessionController {
  async fn login(
    State(state): State<AppState>,
    Json(body): Json<JWTBody>
  ) -> HttpResult<Json<SessionData>> {
    let mut db = state.postgres.get()?;

    Ok(Json(SessionService::login(&mut db, body.token)
      .await?))
  }

  async fn join(
    State(state): State<AppState>,
    Json(body): Json<SessionData>
  ) -> HttpResult<()> {
    let mut db = state.postgres.get()?;

    SessionService::join(&mut db, body)?;

    Ok(())
  }

  async fn has_joined(
    State(state): State<AppState>,
    Query(query): Query<HasJoinedQuery>
  ) -> HttpResult<Json<PlayerJoinResponse>> {
    let mut db = state.postgres.get()?;

    Ok(Json(SessionService::has_joined(&mut db, query.username, query.server_id)?))
  }
}

impl Controller<AppState> for SessionController {
  fn register(&self, router: axum::Router<AppState>) -> axum::Router<AppState> {
    router
      .route("/login", post(Self::login))
      .route("/sessionserver/session/minecraft/join", post(Self::join))
      .route("/sessionserver/session/minecraft/hasJoined", get(Self::has_joined))
  }
}