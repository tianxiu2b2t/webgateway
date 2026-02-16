pub mod backlog;

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}

pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
    hex.as_bytes()
        .chunks(2)
        .map(|chunk| u8::from_str_radix(std::str::from_utf8(chunk).unwrap(), 16).unwrap())
        .collect()
}

pub fn repr_bytes(bytes: &[u8]) -> String {
    // if is ascii
    let mut result = String::new();
    for byte in bytes {
        if *byte >= 32 && *byte <= 126 {
            result.push(*byte as char);
        } else {
            result.push_str(&format!("\\x{byte:02x}"));
        }
    }
    result
}

pub fn default_false() -> bool {
    false
}

pub fn default_true() -> bool {
    true
}
