use crate::block::Block;
use std::time::{SystemTime, UNIX_EPOCH};

pub(crate) struct BlockChain {
   pub chain: Vec<Block>
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
}