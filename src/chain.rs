use super::*;
use std::collections::HashSet;

//Error types for chain verification
#[derive(Debug)]
pub enum BlockValidationErr {
    MismatchedIndex,
    InvalidHash,
    AchronologicalTimestamp,
    MismatchedPreviousHash,
    InvalidGenesisBlockFormat,
    InvalidInput,
    InsufficientInputValue,
    InvalidCoinbaseTransaction,
}

//The blockchain - Chain of blocks in one place
pub struct Chain {
    pub blocks: Vec<Block>,
    unspent_outputs: HashSet<Hash>,
}

impl Chain {
    pub fn new() -> Self {
        Chain {
            blocks: vec![],
            unspent_outputs: HashSet::new()
        }
    }

    pub fn update_with_block (&mut self, block: Block) -> Result<(), BlockValidationErr> {
        let i = self.blocks.len();
        //Verify index
        if block.index != i as u32 {
            return Err(BlockValidationErr::MismatchedIndex);
        }
        //Verify nonce meets PoW in hash
        else if !block.verify_pow() {
            return Err(BlockValidationErr::InvalidHash);
        } 
        else if i != 0 {
            //Successor blocks
            let prev_block = &self.blocks[i-1];
            if block.timestamp <= prev_block.timestamp {
                return Err(BlockValidationErr::AchronologicalTimestamp);
            }
            else if block.prev_block_hash != prev_block.block_hash {
                return Err(BlockValidationErr::MismatchedPreviousHash);
            }
        } 
        else {
            //Genesis block
            if block.prev_block_hash != vec![0; 32] {
                return Err(BlockValidationErr::InvalidGenesisBlockFormat);
            }
        }

        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            if !coinbase.is_coinbase() {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction)
            }

            let mut block_spent : HashSet<Hash> = HashSet::new();
            let mut block_created : HashSet<Hash> = HashSet::new();
            let mut total_fee = 0;

            for transaction in transactions {
                let input_hashes = transaction.input_hashes();

                if !(&input_hashes - &self.unspent_outputs).is_empty() || 
                    !(&input_hashes & &block_spent).is_empty() {
                    return Err(BlockValidationErr::InvalidInput);
                }

                let input_value = transaction.input_valuesum();
                let output_value = transaction.output_valuesum();

                if output_value > input_value {
                    return Err(BlockValidationErr::InsufficientInputValue);
                }

                let fee = input_value - output_value;
                total_fee += fee;

                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hashes())

            }

            if coinbase.output_valuesum() < total_fee {
                return Err(BlockValidationErr::InvalidCoinbaseTransaction)
            } else {
                block_created.extend(coinbase.output_hashes())
            }

            self.unspent_outputs.retain(|output| !block_spent.contains(output));
            self.unspent_outputs.extend(block_created);
        }

        Ok(())
    }
}