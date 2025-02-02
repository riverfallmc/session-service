use anyhow::Result;
use num_bigint::BigInt;
use sha1::{Digest, Sha1};

use super::time::TimeService;

pub struct RandomService;

impl RandomService {
  fn digest(
    username: String
  ) -> Vec<u8> {
    let mut hasher = Sha1::new();
    hasher.update(username.as_bytes());
    hasher.finalize().to_vec()
  }

  pub fn generate_uuid(
    username: String
  ) -> String {
    BigInt::from_signed_bytes_be(&Self::digest(username))
      .to_str_radix(16)
  }

  pub fn generate_access(
    username: String
  ) -> Result<String> {
    let time = &TimeService::get_timestamp()?
      .to_string();

    Ok(Self::generate_uuid(username + time))
  }
}