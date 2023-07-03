use serde::{Serialize, Deserialize};
use serde_json;
use crate::kibi::utils::get_timestamp;

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
  pub index: i64,
  pub nonce: i64,
  pub transactions: Vec<String>, // this is the data
  pub timestamp: u64,
  pub hash: String,
  pub prev_hash: String,
}

impl Block {
  pub fn new(index: i64, prev_hash: String, hash: Option<String>) -> Block {
    Block {
        index,
        transactions: vec![],
        timestamp: get_timestamp(),
        hash: hash.unwrap_or("0".to_string()),
        prev_hash,
        nonce: 0
    }
  }

  pub fn compute_hash(&mut self) -> String {
    // Update its hash (compute using the entire block data)
    let stringified_block = serde_json::to_string(&self).unwrap();
    self.hash = sha256::digest(stringified_block);
    // Return the current hash
    return self.hash.to_string();
  }
}