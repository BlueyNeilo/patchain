use super::*;

//Error types for chain verification
pub enum ChainValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidGenesisBlockFormat,
}

//The blockchain - Chain of blocks in one place
pub struct Chain {
    pub blocks: Vec<Block>,
}

impl Chain {
    pub fn verify (&self) -> Result<(), ChainValidationErr> {
        for (i, block) in self.blocks.iter().enumerate() {
            //Verify index
            if block.index != i as u32 {
                return Err(ChainValidationErr::MismatchedIndex);
            }
            //Verify nonce meets PoW in hash
            else if !block.verify_pow() {
                return Err(ChainValidationErr::InvalidHash);
            } 
            else if i != 0 {
                //Successor blocks
                let prev_block = &self.blocks[i-1];
                if block.timestamp <= prev_block.timestamp {
                    return Err(ChainValidationErr::AchronologicalTimestamp);
                }
                else if block.prev_block_hash != prev_block.block_hash {
                    return Err(ChainValidationErr::MismatchedPreviousHash);
                }
            } 
            else {
                //First block
                if block.prev_block_hash != vec![0; 32] {
                    return Err(ChainValidationErr::InvalidGenesisBlockFormat);
                }
            }
        }

        Ok(())
    }
}