use sha1::{Digest, Sha1};

pub struct HasherService;

impl HasherService {
  pub fn sha1_bytes(bytes: &Vec<u8>) -> String {
    let mut hasher = Sha1::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
  }
}
