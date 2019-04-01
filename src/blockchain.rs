use super::*;
use std::collections::HashSet;

#[derive(Debug)]
pub enum BlockValidationError {
    MismatchedIndex,
    InvalidDifficulty,
    InvalidHash,
    AchronologicalTimestamps,
    MismatchedPreviousHash,
    InvalidGenesisBlock,
    InvalidInput,
    InsuficientInputValue,
    InvalidCoinbaseTransaction,
    InsuficientCoinbaseTransaction,
    InvalidTransactionTimestamp,
    InvalidTransactionBlock,
}

pub struct Blockchain {
    pub blocks: Vec<Block>,
    unspent_outputs: HashSet<Hash>,
}

impl Blockchain {
    pub fn new() -> Self {
        Self {
            blocks: vec![],
            unspent_outputs: HashSet::new(),
        }
    }

    pub fn update_with_block(&mut self, block: Block) -> Result<(), BlockValidationError> {
        let last_index = self.blocks.len();

        if block.index != (last_index as u32) {
            return Err(BlockValidationError::MismatchedIndex);
        }

        if !block::check_difficulty(&block.hash, block.difficulty) {
            return Err(BlockValidationError::InvalidDifficulty);
        }

        if last_index > 0 {
            let prev_block = &self.blocks[last_index - 1];
            if &block.timestamp <= &prev_block.timestamp {
                return Err(BlockValidationError::AchronologicalTimestamps);
            }

            if &block.prev_block_hash != &prev_block.hash {
                return Err(BlockValidationError::MismatchedPreviousHash);
            }
        } else {
            if block.prev_block_hash != [0; 32] {
                return Err(BlockValidationError::InvalidGenesisBlock);
            }
        }

        if let Some((coinbase, transactions)) = block.transactions.split_first() {
            if !coinbase.is_coinbase() {
                return Err(BlockValidationError::InvalidCoinbaseTransaction);
            }

            let mut block_spent: HashSet<Hash> = HashSet::new();
            let mut block_created: HashSet<Hash> = HashSet::new();

            let mut total_fee = 0;

            for transaction in transactions {
                let input_hashes = transaction.input_hashes();

                if !(&input_hashes - &self.unspent_outputs).is_empty()
                    || !(&input_hashes & &block_spent).is_empty()
                {
                    return Err(BlockValidationError::InvalidInput);
                }

                let input_value = transaction.input_value();
                let output_value = transaction.output_value();

                if input_value < output_value {
                    return Err(BlockValidationError::InsuficientInputValue);
                }

                if transaction.timestamp > block.timestamp {
                    return Err(BlockValidationError::InvalidTransactionTimestamp);
                }

                let fee = input_value - output_value;
                total_fee += fee;

                block_spent.extend(input_hashes);
                block_created.extend(transaction.output_hashes());

                println!("Verified transaction: {}", &transaction);
            }

            if coinbase.output_value() < total_fee {
                return Err(BlockValidationError::InsuficientCoinbaseTransaction);
            } else {
                println!("Verified coinbase transaction: {}", &coinbase);
                block_created.extend(coinbase.output_hashes());
            }

            self.unspent_outputs
                .retain(|output| !block_spent.contains(output));
            self.unspent_outputs.extend(block_created);
        }

        println!("Verified block: {}", &block);

        self.blocks.push(block);

        Ok(())
    }
}
