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

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub block_hash: Hash,
    pub prev_block_hash: Hash,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub difficulty: u8,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Block[{}]: {} at: {} with: {} transactions", 
        &self.index,
        &hex::encode(&self.block_hash),
        &self.timestamp,
        &self.transactions.len())
    }
}

impl Block {
    pub fn new(index: u32, timestamp: u128, prev_block_hash: Hash, nonce: u64, transactions: Vec<Transaction>, difficulty: u8) -> Self {
        Block {
            index,
            timestamp,
            block_hash: vec![0; 32],
            prev_block_hash,
            nonce,
            transactions,
            difficulty,
        }
    }
    //Need [difficulty] number of 0s to satisfy nonce
    pub fn mine(&mut self) {
        for nonce_attempt in 0..(u64::max_value()) {
            self.nonce = nonce_attempt;

            if self.verify_pow() {
                self.rehash();

                return;
            }
        }
    }

    //Verify that the proof of work (PoW) has been done to find a valid nonce
    pub fn verify_pow(&self) -> bool {
        self.hash().ends_with(vec![0; self.difficulty as usize].as_slice())
    }

    //Refresh the hash to reflect the block's information
    fn rehash(&mut self) {
        self.block_hash = self.hash()
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Hash {
        vec![
            self.index.bytes(),
            self.timestamp.bytes(),
            self.prev_block_hash.bytes(),
            self.nonce.bytes(),
            self.transactions.bytes(),
            self.difficulty.bytes(),
        ].bytes()
    }
}