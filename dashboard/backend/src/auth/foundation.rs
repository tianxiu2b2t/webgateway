use std::sync::LazyLock;

use base64::Engine;

pub fn generate_random_secret() -> String {
    let mut bytes = [0u8; 32];
    rand::fill(&mut bytes);
    base64::prelude::BASE64_STANDARD.encode(bytes)
}

pub static DEFAULT_ADMIN_USERNAME: LazyLock<String> =
    LazyLock::new(|| std::env::var("USERNAME").unwrap_or("admin".to_string()));
