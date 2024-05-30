pub fn convert(raw_password: &str) -> String {
    use hmac_sha512::Hash as SHA512;

    let password = SHA512::hash(raw_password.as_bytes());
    hex::encode(password)
}
