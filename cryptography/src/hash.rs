use openssl::sha::sha256;

pub fn string_to_hash(message: String) -> String {
    let hash = sha256(message.as_bytes());
    let mut res: String = String::new();
    for b in hash.iter() {
        res.push(*b as char);
    }
    res
}
