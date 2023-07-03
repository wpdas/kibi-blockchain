use sha256;
use std::time::SystemTime;

pub fn hash_generator(data: String) -> String {
  return sha256::digest(data);
}

pub fn get_timestamp() -> u64 {
  let time = SystemTime::now();
  let duration = time.duration_since(SystemTime::UNIX_EPOCH).unwrap();
  duration.as_secs()
}

// Difficulty of PoW algorithm
pub const DIFFICULTY: usize = 5;