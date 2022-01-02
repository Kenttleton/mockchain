use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    id: String,
}

impl Account {
    pub fn new() -> Self {
        Account { id: String::new() }
    }
}
