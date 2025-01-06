use rust_blockchain::BlockHash;
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct Block {
    pub timestamp: u128,
    pub block_hash: BlockHash,
    pub previous_block_hash: BlockHash,
    pub payload: String
}

impl Block {
    pub fn new( 
        timestamp: u128,
        previous_block_hash: BlockHash,
        payload: String ) -> Self {
        let mut block = Block {
            timestamp, 
            block_hash: vec![0; 256],
            previous_block_hash,
            payload
        };

        block.block_hash = block.create_block_hash();

        block
    }
}

impl Block {

    fn create_block_hash(&self) -> Vec<u8> {
        let mut hashing_handler = Sha256::new();
        hashing_handler.update(self.timestamp.to_be_bytes());
        // Clone here for now as we do not need specific ownership, but if using various hashing algorithms
        // And length gets long we may affect performance, so take ownership in future if this is a problem
        hashing_handler.update(self.previous_block_hash.clone());
        hashing_handler.update(self.payload.as_bytes());
        let test = hashing_handler.finalize();
        test.to_vec()
    }
}


