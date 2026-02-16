use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthPostBody {
    pub username: String,
    pub totp: String,
}
