use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::Result;

pub struct TimeService;

impl TimeService {
  pub fn get_timestamp() -> Result<u64> {
    let start = SystemTime::now();
    let duration = start.duration_since(UNIX_EPOCH)?;
    Ok(duration.as_secs())
  }
}