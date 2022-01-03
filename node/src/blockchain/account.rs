use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Account {
    id: [u8; 32],
}

impl Account {
    pub fn new() -> Self {
        Account { id: [0; 32] }
    }
}
