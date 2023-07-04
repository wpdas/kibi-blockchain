use serde_json::Value;
use sha256;
use std::time::SystemTime;

use super::block::{BlockJson, Block};

pub fn hash_generator(data: String) -> String {
  return sha256::digest(data);
}

pub fn get_timestamp() -> u64 {
  let time = SystemTime::now();
  let duration = time.duration_since(SystemTime::UNIX_EPOCH).unwrap();
  duration.as_secs()
}

/**
 * Converts Block (stringified JSON transactions) to BlockJson data (parse JSON transactions)
 */
pub fn block_to_blockjson(block: Block) -> BlockJson {
  // decode transactions
  let mut transaction_json: Vec<Value> = vec![];

  for transaction in block.transactions {
    transaction_json.push(serde_json::from_str(transaction.as_str()).unwrap());
  }

  // create a BlockJson data
  BlockJson {
    index: block.index,
    nonce: block.nonce,
    timestamp: block.timestamp,
    hash: block.hash,
    prev_hash: block.prev_hash,
    // update with the decoded transactions (json format)
    transactions: transaction_json,
  }
}

// Difficulty of PoW algorithm
pub const DIFFICULTY: usize = 5;