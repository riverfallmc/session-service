use std::fs;
use anyhow::Result;
use base64::{engine::general_purpose::STANDARD, Engine};
use once_cell::sync::Lazy;
use openssl::{hash::MessageDigest, pkey::PKey, rsa::Rsa, sign::Signer};

pub struct SignerService;

static PRIVATE_KEY: Lazy<PKey<openssl::pkey::Private>> = Lazy::new(|| {
  let content = fs::read("data/private.pem")
    .expect("No data/private.pem");

  let rsa = Rsa::private_key_from_pem(&content)
    .expect("Invalid private key");

  PKey::from_rsa(rsa).expect("Failed to create PKey")
});

impl SignerService {
  pub fn sign(data: &str) -> Result<String> {
    let mut signer = Signer::new(MessageDigest::sha256(), &*PRIVATE_KEY)?;
    signer.update(data.as_bytes())?;
    let signature = signer.sign_to_vec()?;

    Ok(STANDARD.encode(signature))
  }
}
