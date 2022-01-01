use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Block {
    previous_hash: String,
    amount: f32,
    to_account: String,
    from_account: String,
    creation_date: String,
    transaction_date: String,
}

impl Block {
    pub fn new() -> Self {
        Block {
            previous_hash: String::new(),
            amount: f32::MIN,
            to_account: String::new(),
            from_account: String::new(),
            creation_date: String::new(),
            transaction_date: String::new(),
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
