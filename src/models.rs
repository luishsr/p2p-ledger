use std::time::{SystemTime, UNIX_EPOCH};
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Transaction {
    pub(crate) sender: String,
    pub(crate) recipient: String,
    pub(crate) amount: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    timestamp: u64,
    transactions: Vec<Transaction>,
    previous_hash: String,
    hash: String,
}

impl Block {
    pub fn new(transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut block = Block {
            timestamp,
            transactions,
            previous_hash,
            hash: "".to_string(),
        };
        block.hash = block.calc_hash();
        block
    }

    pub fn calc_hash(&self) -> String {
        let bytes = serde_json::to_vec(self).unwrap();
        let result = Sha256::digest(&bytes);
        hex::encode(result)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain {
    pub(crate) chain: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis_block = Block::new(vec![], "0".to_string());
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    pub fn add_block(&mut self, transactions: Vec<Transaction>) {
        let prev_hash = self.chain.last().unwrap().hash.clone();
        let block = Block::new(transactions, prev_hash);
        self.chain.push(block);
    }
}
