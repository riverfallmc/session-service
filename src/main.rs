use controller::{authlib::AuthlibController, session::SessionController, skincape::SkinCapeController};
use dixxxie::{
  axum::{self, Router}, connection::{establish_connection, DbPool}, controller::ApplyControllerOnRouter, response::HttpResult, setup
};

mod repository;
mod controller;
mod service;
mod models;
mod schema;

#[allow(unused)]
#[derive(Clone)]
struct AppState {
  postgres: DbPool
}

#[tokio::main]
async fn main() -> HttpResult<()> {
  setup()?;

  let state = AppState {
    postgres: establish_connection()?
  };

  let router = Router::new()
    .apply_controller(AuthlibController)
    .apply_controller(SessionController)
    .apply_controller(SkinCapeController)
    .with_state(state);

  let listener = tokio::net::TcpListener::bind("0.0.0.0:80")
    .await?;

  Ok(axum::serve(listener, router).await?)
}