use super::*;
pub struct Chain {
    pub blocks: Vec<Block>,
}

impl Chain {
    pub fn verify (&self) -> bool {
        for (i, block) in self.blocks.iter().enumerate() {
            //Verify index
            if block.index != i as u32 {
                println!("Index mismatch {} != {}", &block.index, i);
                return false
            }
            //Verify nonce meets PoW in hash
            else if !block.verify_pow() {
                println!("PoW difficulty failed");
                return false
            } 
            else if i != 0 {
                //Successor blocks
                let prev_block = &self.blocks[i-1];
                if block.timestamp <= prev_block.timestamp {
                    println!("Time did not increase");
                    return false
                }
                else if block.prev_block_hash != prev_block.block_hash {
                    println!("Hash mismatch");
                    return false
                }

            } 
            else {
                //First block
                if block.prev_block_hash != vec![0; 32] {
                    println!("First block prev_block_hash invalid");
                    return false
                }
            }
        }
        true
    }
}