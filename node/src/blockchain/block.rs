use crate::blockchain::transaction::Transaction;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    previous_hash: String,
    creation_date: String,
    transactions: Vec<Transaction>,
}

impl Block {
    pub fn new() -> Self {
        Block {
            previous_hash: String::new(),
            creation_date: String::new(),
            transactions: Vec::new(),
        }
    }
    pub fn to_str(&self) -> String {
        match serde_json::to_string(&self) {
            Ok(block) => block,
            Err(e) => e.to_string(),
        }
    }
    pub fn from_str(self, data: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(data)
    }
}
