pub fn handler(buf: [u8; 1000000]) -> [u8; 1000000] {
    let mut message = String::new();
    let mut res: [u8; 1000000] = [0; 1000000];
    for x in buf.iter() {
        message.push(*x as char);
    }
    if message == "ping" {
        pad_from_str(&mut res, "pong");
    }
    println!("Message: {}", message);
    res
}

fn pad_from_str(arr: &mut [u8], s: &str) {
    let bytes = s.as_bytes();
    if bytes.len() <= 1000000 {
        let mut i = 0;
        for b in bytes {
            arr[i] = *b;
            i = i + 1;
        }
    } else {
        pad_from_str(arr, "ERROR: Padding parser failure.");
    }
}
