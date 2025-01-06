use crate::block::Block;
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) struct BlockChain {
   chain: Vec<Block>
}

impl BlockChain {
    pub fn new() -> Self {
        Self {
            chain: vec![Block::new(BlockChain::generate_current_timestamp(), vec![0;32], "Genesis_Block".to_owned())]
        }
    }

    fn generate_current_timestamp() -> u128 {
        SystemTime::now().duration_since(UNIX_EPOCH).expect("Could not get current time").as_millis()
    }

    pub fn add_new_block(&mut self, data: String) {
        let previous_block_hash = self.chain.last().expect("Could not fetch previous hash").previous_block_hash.clone();
        self.chain.push(Block::new(BlockChain::generate_current_timestamp(), previous_block_hash, data));
    }
}