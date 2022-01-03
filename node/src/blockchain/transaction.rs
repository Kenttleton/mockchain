use crate::blockchain::account::Account;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Transaction {
    amount: f32,
    to: Account,
    from: Account,
}

impl Transaction {
    pub fn new() -> Self {
        Transaction {
            amount: 0.0_f32,
            to: Account::new(),
            from: Account::new(),
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
