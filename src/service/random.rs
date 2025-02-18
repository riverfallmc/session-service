use anyhow::Result;
use rand::{rng, RngCore};
use sha2::{Sha256, Digest};
use super::time::TimeService;

pub struct RandomService;

impl RandomService {
  // always must be md5!! (32 symbols)
  pub fn generate_uuid(
    username: String
  ) -> Result<String> {
    let timestamp = TimeService::get_timestamp()?.to_string();
    let data = format!("{username}{timestamp}");
    Ok(hex::encode(md5::compute(data).0))
  }

  pub fn generate_access_token() -> String {
    let mut rng = rng();
    let mut bytes = [0u8; 16];
    rng.fill_bytes(&mut bytes);

    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
  }
}

mod tests {
  #[test]
  fn uuid() -> anyhow::Result<()> {
    use crate::service::random::RandomService;
    let generated_uuid = RandomService::generate_uuid(String::from("smokingplaya"))?;

    println!("uuid: {}", generated_uuid);
    assert!(generated_uuid.len() == 32);

    Ok(())
  }
}