use adjust::{controller::Controller, response::{HttpMessage, HttpResult, NonJsonHttpResult}};
use axum::{extract::{Path, Query}, routing::post, Json, Router};
use serde::Deserialize;
use crate::{service::{invite::InviteService, user::UserService}, AppState};

#[derive(Deserialize)]
struct InviteQuery {
  sender: i32,
  server: i32
}

pub struct InviteController;

impl InviteController {
  async fn invite(
    Path(user_id): Path<i32>,
    Query(query): Query<InviteQuery>
  ) -> HttpResult<HttpMessage> {
    UserService::check_can_invite(user_id, query.sender)
      .await?;

    InviteService::send_invite(user_id, query.sender, query.server);

    Ok(Json(
      HttpMessage::new("Приглашение отправлено")
    ))
  }
}

impl Controller<AppState> for InviteController {
  fn new() -> anyhow::Result<Box<Self>> {
    Ok(Box::new(Self))
  }

  fn register(&self, router: Router<AppState>) -> Router<AppState> {
    router
      .nest("/invite",
      Router::new()
        .route("/{id}", post(Self::invite))
    )
  }
}