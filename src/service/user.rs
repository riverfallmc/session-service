use std::sync::LazyLock;
use adjust::{load_env, response::{HttpError, NonJsonHttpResult}};
use reqwest::{Client, StatusCode};

load_env!(USER_URL);

static CLIENT: LazyLock<Client> = LazyLock::new(Client::default);

pub struct UserService;

impl UserService {
  pub async fn check_can_invite(
    user: i32,
    other_user: i32,
  ) -> NonJsonHttpResult<()> {
    CLIENT.get(format!("http://{}/privacy/check/{user}?actor={other_user}&type=INVITE", *USER_URL))
      .send()
      .await?
      .error_for_status()
      .map_err(|_| HttpError::new("Вы не можете пригласить в игру этого пользователя", Some(StatusCode::FORBIDDEN)))?;

    Ok(())
  }
}