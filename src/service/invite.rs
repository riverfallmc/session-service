use std::sync::LazyLock;
use adjust::load_env;
use reqwest::Client;
use serde_json::json;

load_env!(WSS_URL);

static CLIENT: LazyLock<Client> = LazyLock::new(Client::default);

pub struct InviteService;

impl InviteService {
  pub fn send_invite(
    user_id: i32,
    sender_id: i32,
    server_id: i32
  ) {
    tokio::spawn(async move {
      #[allow(unused)]
      CLIENT.post(format!("http://{}/send/{user_id}?type=INVITE", *WSS_URL))
        .header("Content-Type", "application/json")
        .body(json!({
          "id": sender_id,
          "server": server_id
        }).to_string())
        .send()
        .await;
    });
  }
}