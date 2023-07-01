use sha256;

fn hash_generator(data: String) -> String {
    return sha256::digest(data);
}

#[derive(Debug)]
struct Block {
    data: String,
    hash: String,
    prev_hash: String,
}

impl Block {
    fn compute_hash(&mut self) -> String {
        self.hash = sha256::digest(self.data.to_string() + &self.prev_hash.to_string() + &self.nonce.to_string());
        return self.hash.to_string();
    }
}

struct Blockchain {
    chain: Vec<Block>,
}

impl Blockchain {
    // Init the blockchain with a genesis block
    fn init(&mut self) {
        let hash_last = hash_generator("last_gen".to_string());
        let hash_first = hash_generator("first_gen".to_string());

        let genesis_block = Block {
            data: "gen_data".to_string(),
            hash: hash_first,
            prev_hash: hash_last,
        };

        self.chain.push(genesis_block);
    }

    // Add new block
    fn add_block(&mut self, data: String) {
        let prev_hash = &self.chain.last().unwrap().hash;
        let hash = hash_generator(data.to_string() + prev_hash.as_str());
        let mut block = Block { data, hash, prev_hash: prev_hash.to_owned()};

        // Add the block to the chain
        self.chain.push(block)
    }

    // Get the last block
    fn last_block(&self) -> &Block {
        self.chain.last().unwrap()
    }
}

fn main() {
    let mut blch = Blockchain{chain: Vec::new()};
    blch.init();

    blch.add_block("A".to_string());
    blch.add_block("B".to_string());
    blch.add_block("C".to_string());

    for block in &blch.chain {
        println!("{:?}", block);
    }

    // let last_block = blch.last_block();
    // println!("{:?}", last_block)
}
