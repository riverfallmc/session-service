use adjust::{controller::Controller, response::HttpResult};
use axum::{extract::{Path, Query, State}, routing::{get, post}, Json};
use serde::{Deserialize, Serialize};
use crate::{models::{session::{PlayerJoinResponse, PlayerJoinResponseTextures, SessionData}, users::GlobalUserData}, service::session::SessionService, AppState};

#[derive(Serialize, Deserialize)]
struct HasJoinedQuery {
  pub username: String,
  #[serde(rename="serverId")]
  pub server_id: String,
}

#[derive(Serialize, Deserialize)]
struct ProfileQuery {
  pub unsigned: Option<bool>,
}

pub struct SessionController;

impl SessionController {
  async fn login(
    State(state): State<AppState>,
    Query(user): Query<GlobalUserData>
  ) -> HttpResult<SessionData> {
    let mut db = state.postgres.get()?;

    Ok(Json(SessionService::login(&mut db, user)
      .await?))
  }

  async fn join(
    State(state): State<AppState>,
    Json(body): Json<SessionData>
  ) -> HttpResult<()> {
    let mut db = state.postgres.get()?;

    #[allow(unused)]
    SessionService::join(&mut db, body)?;

    Ok(Json(()))
  }

  async fn profile(
    Path(username): Path<String>,
    State(state): State<AppState>,
  ) -> HttpResult<PlayerJoinResponseTextures> {
    let mut db = state.postgres.get()?;

    Ok(Json(SessionService::profile(&mut db, &username)?))
  }

  async fn has_joined(
    State(state): State<AppState>,
    Query(query): Query<HasJoinedQuery>
  ) -> HttpResult<PlayerJoinResponse> {
    let mut db = state.postgres.get()?;

    Ok(Json(SessionService::has_joined(&mut db, query.username, query.server_id)
      .await?))
  }

  async fn get_profile(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
    Query(query): Query<ProfileQuery>
  ) -> HttpResult<PlayerJoinResponse> {
    let mut db = state.postgres.get()?;

    Ok(Json(SessionService::get_profile(&mut db, uuid, query.unsigned.unwrap_or_default())
      .await?))
  }
}

impl Controller<AppState> for SessionController {
  fn new() -> anyhow::Result<Box<Self>> {
    Ok(Box::new(Self))
  }

  fn register(&self, router: axum::Router<AppState>) -> axum::Router<AppState> {
    router
      .route("/login", post(Self::login))
      .route("/profile/{username}", get(Self::profile))
      .route("/sessionserver/session/minecraft/join", post(Self::join))
      .route("/sessionserver/session/minecraft/hasJoined", get(Self::has_joined))
      .route("/sessionserver/session/minecraft/profile/{uuid}", get(Self::get_profile))
  }
}