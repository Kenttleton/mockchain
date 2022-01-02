use crate::server::addresses::{Destination, NODE};
pub mod account;
pub mod transaction;

pub fn handler(message: &str) -> (String, Destination) {
    let mut response: String = "PONG".to_string();
    let mut transaction = transaction::Transaction::new();
    let result = transaction.from_str(message);
    match result {
        Ok(b) => transaction = b,
        Err(e) => response = e.to_string(),
    }
    println!("Message: {}", message);
    (response, NODE)
}
