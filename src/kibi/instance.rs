use once_cell::sync::Lazy;

use super::{
  blockchain::{Blockchain, MineReturnOptions},
  block::Block
};

/**
 * A blockchian instance to be used globally
 * 
 * This was the way I found to use a Blockchain instance globally:
 * https://stackoverflow.com/questions/27791532/how-do-i-create-a-global-mutable-singleton
 */
static mut BLOCKCHAIN: Lazy<Blockchain> = Lazy::new(|| Blockchain::new());
// static mut ACCOUNTS: Lazy<AccountId> = Lazy::new(|| AccountId::init());

pub struct BlockchainInstance {}

impl BlockchainInstance {
  pub fn add_new_transaction(transaction: String) {
    unsafe {
      BLOCKCHAIN.add_new_transaction(transaction)
    }
  }

  pub fn get_chain() -> Vec<Block> {
    unsafe {
      return BLOCKCHAIN.chain.to_vec()
    }
  }

  pub fn mine() -> MineReturnOptions {
    unsafe {
      return BLOCKCHAIN.mine()
    }
  }

  // pub fn create_account(account: String) {
  //   AccountId::new(account);
  // }
}