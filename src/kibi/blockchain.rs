use crate::kibi::block::Block;
use crate::kibi::utils::{hash_generator, DIFFICULTY};

// A way to inform the kinds of data for a given method
#[derive(Debug)]
pub enum MineReturnOptions {
    BOOL(bool),
    I64(i64),
}

pub struct Blockchain {
  pub chain: Vec<Block>,
  pub unconfirmed_transactions: Vec<String>
}

impl Blockchain {
  // Auto create an instance of Blockchain and init it
  pub fn new() -> Blockchain {
    let mut instance = Blockchain {
        chain: Vec::new(),
        unconfirmed_transactions: Vec::new()
    };
    instance.init();
    return instance;
  }

  // Init the blockchain with a genesis block
  fn init(&mut self) {
      let hash_first = hash_generator("first_gen".to_string());
      let genesis_block = Block::new(0, "0".to_string(), Some(hash_first), None);

      self.chain.push(genesis_block);
  }

  // Get the last block
  fn last_block(&self) -> &Block {
      self.chain.last().unwrap()
  }

  /**
   * Adds the block to the chain after verification.
   * 
   * Verification includes:
   * - Checking if the proof is valid.
   * - The previous_hash referred in the block and the hash of latest block
   *   in the chain match
   */
  fn add_block(&mut self, block: &Block, proof: String) -> bool {
      let previous_hash = &self.last_block().hash;

      if previous_hash != &block.prev_hash {
          return false;
      }

      if !self.is_valid_proof(&block, proof) {
          return false;
      }

      // NOTE: idk why, but this is failing because what this
      // function receives is a &Block, and the chain is a kind of
      // Vec<Block>. So the compiler is reclaiming about it.

      // So, the way out was recreating a new Block using
      // the block parameter data info
      self.chain.push(Block {
          index: block.index,
          nonce: block.nonce,
          transactions: block.transactions.clone(),
          timestamp: block.timestamp,
          hash: block.hash.clone(),
          prev_hash: block.prev_hash.clone()
      });

      true
  }

  /**
   * Check if block_hash is valid hash of block and satisfies the difficulty criteria.
   */
  fn is_valid_proof(&self, block: &Block, block_hash: String) -> bool {
      // sets the difficulty chars. e.g.: 000 if DIFFICULTY is 3
      let difficulty_chars = "0".repeat(DIFFICULTY);

      block_hash.starts_with(difficulty_chars.as_str()) && block_hash == block.hash
  }

  /**
   * Function that tries different values of the nonce to get a hash
   * that satisfies our difficulty criteria.
   */
  fn proof_of_work(&self, block: &mut Block) -> String {
      // initial computed_hash (the initial block.hash)
      let mut computed_hash = String::from(&block.hash);

      // sets the difficulty chars. e.g.: 000 if DIFFICULTY is 3
      let difficulty_chars = "0".repeat(DIFFICULTY); // NOTE: REPEATED

      // check logic
      while !computed_hash.starts_with(difficulty_chars.as_str()) {
          block.nonce += 1; // add 1 to change the hash
          computed_hash = block.compute_hash();
      }

      computed_hash
  }

  pub fn add_new_transaction(&mut self, transaction: String) {
      self.unconfirmed_transactions.push(transaction);
  }

  pub fn mine(&mut self) -> MineReturnOptions {
      if self.unconfirmed_transactions.is_empty() {
          return MineReturnOptions::BOOL(false);
      }

      let last_block = self.last_block();

      let mut new_block = Block::new(
        last_block.index + 1,
        last_block.hash.clone(),
        None,
        Some(self.unconfirmed_transactions.clone()),
      );

      let proof = self.proof_of_work(&mut new_block);
      self.add_block(&new_block, proof);

      // clear unconfirmed transactions
      self.unconfirmed_transactions.clear();

      return MineReturnOptions::I64(new_block.index);
  }
}