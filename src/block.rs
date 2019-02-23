//The block datastructure of the blockchain

/*
Blocks contain this information:

Index - this block's location within the list of blocks
Payload - any relevant information or events that have occurred for/in the block
Timestamp - gives our blockchain a sense of time
Nonce - a special number used for mining (PoW verification)
Previous block hash - cryptographic fingerprint of previous block
Hash - cryptographic fingerprint of all of the above data concatenated together
*/
use std::{fmt::{self,*},};
use super::*;

type BlockHash = Vec<u8>;

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub block_hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: u64,
    pub payload: String,
    pub difficulty: u8,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block[{}]: {} at: {} with: {}", 
        &self.index,
        &hex::encode(&self.block_hash),
        &self.timestamp,
        &self.payload)
    }
}

impl Block {
    pub fn new(index: u32, timestamp: u128, prev_block_hash: BlockHash, nonce: u64, payload: String, difficulty: u8) -> Self {
        Block {
            index,
            timestamp,
            block_hash: vec![0; 32],
            prev_block_hash,
            nonce,
            payload,
            difficulty,
        }
    }
    //Need [difficulty] number of 0s to satisfy nonce
    pub fn mine(&mut self) {
        for nonce_attempt in 0..(u64::max_value()) {
            self.nonce = nonce_attempt;
            let hash = self.hash();

            if hash.ends_with(vec![0; self.difficulty as usize].as_slice()) {
                self.block_hash = hash;
                return;
            }
        }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&self.index.to_be_bytes());
        bytes.extend(&self.timestamp.to_be_bytes());
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&self.nonce.to_be_bytes());
        bytes.extend((&self.payload).as_bytes());
        bytes.extend(&self.difficulty.to_be_bytes());
        bytes
    }
}