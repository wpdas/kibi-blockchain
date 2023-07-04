use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionData {
  pub from: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub to: Option<String>,
  pub content: String,
  pub timestamp: Option<u64>,
}