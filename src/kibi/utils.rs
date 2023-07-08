use serde::Deserialize;
use serde_json::Value;
use sha256;
use std::{time::SystemTime, collections::HashMap};

use crate::kibi::types::ContractTransactionData;

use super::{block::{BlockJson, Block}, instance::BlockchainInstance, types::{Kib, KibFields}};

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

/**
 * Get the most updated Kib fields info from chain
 */
pub fn get_kib_from_chain () -> Kib{
  let mut chain = BlockchainInstance::get_chain();
  chain.reverse();

  for block in chain {

    // decode transactions
    let block_json = block_to_blockjson(block);

    for tx in block_json.transactions {
      if tx["kib"].is_object() {
        let restored_kib: Kib = serde_json::from_value(tx).unwrap();
        return restored_kib;
      }
    }
  }

  Kib { kib: KibFields { accounts: HashMap::new() } }
}

/**
 * Get the most updated Contract fields info from chain
 */
pub fn get_contract_from_chain (contract_id: String) -> Option<ContractTransactionData>{
  let mut chain = BlockchainInstance::get_chain();
  chain.reverse();

  for block in chain {

    // decode transactions
    let block_json = block_to_blockjson(block);

    for tx in block_json.transactions {
      if tx["contract_id"].is_string() {
        let contract: ContractTransactionData = serde_json::from_value(tx).unwrap();
        if contract.contract_id == contract_id {
          return Some(contract);
        }
      }
    }
  }

  None
}

// Difficulty of PoW algorithm
pub const DIFFICULTY: usize = 4;