use crate::server::addresses::{Destination, NODE};
mod block;

pub fn handler(message: &str) -> (String, Destination) {
    let mut response: String = "PONG".to_string();
    let mut block = block::Block::new();
    let result = block.from_str(message);
    match result {
        Ok(b) => block = b,
        Err(e) => response = e.to_string(),
    }
    println!("Message: {}", message);
    (response, NODE)
}
