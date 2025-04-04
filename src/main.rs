use std::sync::Arc;
use adjust::{controllers, controller::Controller, database::{postgres::Postgres, Pool}, main, service::Service};
use controller::{authlib::AuthlibController, invite::InviteController, session::SessionController, skincape::SkinCapeController};

mod repository;
mod controller;
mod service;
mod models;
mod schema;

#[allow(unused)]
#[derive(Default, Clone)]
pub struct AppState {
  postgres: Arc<Pool<Postgres>>
}

#[main]
async fn main() -> Service<AppState> {
  Service {
    name: "Session",
    state: AppState::default(),
    controllers: controllers![AuthlibController, InviteController, SessionController, SkinCapeController],
    ..Default::default()
  }
}