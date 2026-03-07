use anyhow::Result;

pub fn get_totp_code(username: impl Into<String>, secret: impl Into<Vec<u8>>) -> Result<String> {
    Ok(totp_rs::TOTP::new(
        totp_rs::Algorithm::SHA512,
        6,
        1,
        30,
        secret.into(),
        None,
        username.into(),
    )?
    .generate_current()
    .unwrap())
}
