use crate::blockchain::accounts::Account;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    amount: f32,
    to: Account,
    from: Account,
}
