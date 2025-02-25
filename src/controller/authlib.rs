use std::{collections::HashMap, fs::File, io::Read};
use adjust::controller::Controller;
use axum::{routing::get, Json};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use crate::AppState;

#[derive(Serialize, Deserialize)]
struct PublicKey {
  #[serde(rename="publicKey")]
  public_key: String
}

#[derive(Serialize, Deserialize, Clone)]
struct AuthlibMeta {
  #[serde(rename = "serverName")]
  server_name: String,
  #[serde(rename = "implementationName")]
  implementation_name: String,
  #[serde(rename = "implementationVersion")]
  implementation_version: String,
  #[serde(rename = "feature.no_mojang_namespace")]
  feature_no_mojang_namespace: bool,
  links: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Authlib {
  meta: AuthlibMeta,
  #[serde(rename = "skinDomains")]
  skin_domains: Vec<String>,
  #[serde(rename = "signaturePublickey")]
  signature_publickey: String
}

static AUTHLIB: Lazy<Authlib> = Lazy::new(|| {
  Authlib {
    meta: AuthlibMeta {
      server_name: String::from("riverfall.ru ❤"),
      implementation_name: String::from("riverfall.ru session service"),
      implementation_version: env!("CARGO_PKG_VERSION").to_string(),
      feature_no_mojang_namespace: true,
      links: generate_links()
    },
    skin_domains: vec![String::from("riverfall.ru"), String::from("localhost")],
    signature_publickey: read_publickey(),
  }
});

fn generate_links() -> HashMap<String, String> {
  let mut map = HashMap::new();
  map.insert(String::from("homepage"), String::from("https://riverfall.ru"));
  map
}

fn read_publickey() -> String {
  let mut buf = String::new();
  File::open("data/public.pem")
    .expect("data/public.pem is missing")
    .read_to_string(&mut buf).expect("unable to read public.pem");

  buf
}
pub struct AuthlibController;

impl AuthlibController {
  async fn get_authlib_info() -> Json<Authlib> {
    Json(AUTHLIB.clone())
  }

  async fn pubkeys() -> Json<PublicKey> {
    Json(PublicKey {
      public_key: AUTHLIB.clone().signature_publickey
    })
  }
}

impl Controller<AppState> for AuthlibController {
  fn new() -> anyhow::Result<Box<Self>> {
    Ok(Box::new(Self))
  }

  fn register(&self, router: axum::Router<AppState>) -> axum::Router<AppState> {
    router
      .route("/", get(Self::get_authlib_info))
      .route("/minecraftservices/publickeys", get(Self::pubkeys))
  }
}