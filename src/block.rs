use super::*;
use std::fmt::{Display, Formatter};
use std::result::Result;

pub type Hash = Vec<u8>;
pub type Address = String;

pub enum MineError {
    MineError,
}

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: Hash,
    pub prev_block_hash: Hash,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub reward: u64,
    pub difficulty: u128,
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{index: {}, hash: {:32}, previous_hash: {}, nonce: {}, timestamp: {}, transactions: {}, reward: {}}}",
            &self.index,
            &hex::encode_upper(&self.hash),
            &hex::encode_upper(&self.prev_block_hash),
            &self.nonce,
            &self.timestamp,
            &self.transactions
                .iter()
                .map(|i| i.to_string())
                .collect::<String>(),
            &self.reward
        )
    }
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_block_hash: Hash,
        transactions: Vec<Transaction>,
        difficulty: u128,
    ) -> Self {
        assert!(transactions.len() > 0);

        Self {
            index,
            timestamp,
            hash: vec![0; 32],
            prev_block_hash,
            nonce: 0,
            transactions,
            difficulty,
            reward: 0,
        }
    }

    pub fn mine(&mut self) {
        let reward = self.transactions.iter().map(|t| t.fee_value()).sum::<u64>();
        self.reward = reward;

        for nonce in 0..(u64::max_value()) {
            self.nonce = nonce;

            let hash = self.hash();
            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let transactions = self
            .transactions
            .iter()
            .flat_map(|t| t.bytes())
            .collect::<Vec<u8>>();

        let mut bytes = vec![];

        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(transactions);
        bytes.extend(&u64_bytes(&self.reward));
        bytes.extend(&u128_bytes(&self.difficulty));

        bytes
    }
}

pub fn calculate_hash(block: &Block, difficulty: u128) -> Result<Vec<u8>, MineError> {
    let hash = &block.hash();
    if check_difficulty(hash, difficulty) {
        return Ok(hash.to_vec());
    }
    Err(MineError::MineError)
}

pub fn check_difficulty(hash: &Hash, difficulty: u128) -> bool {
    difficulty_bytes_as_u128(&hash) < difficulty
}
