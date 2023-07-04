use rocket::{post, serde::json::Json};

use crate::kibi::{
  types::TransactionData,
  utils::get_timestamp,
  instance::BlockchainInstance
};

// TIP: '_ can be used to set the type as "unknown"

#[post("/", format="json", data="<tx_data>")]
pub fn post(mut tx_data: Json<TransactionData>) -> &'static str {
  // Check fields
  if tx_data.from.is_empty() || tx_data.content.is_empty() {
    return "Invalid transaction data" // 404
  }

  tx_data.timestamp = Some(get_timestamp());

  //Blockchain
  let stringified_tx_data = serde_json::to_string(&tx_data.0).unwrap();
  
  println!("{:?} tx_data:", stringified_tx_data);

  // blockchain.add_new_transaction(stringified_tx_data);
  BlockchainInstance::add_new_transaction(stringified_tx_data);

  "Success"
}