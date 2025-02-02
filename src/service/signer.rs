use std::fs;

use anyhow::Result;
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
  pub fn sign(
    data: String
  ) -> Result<String> {
    let mut signer = Signer::new(MessageDigest::sha256(), &PRIVATE_KEY)?;
    signer.update(data.as_bytes())?;

    Ok(String::from_utf8(signer.sign_to_vec()?)?)
  }
}