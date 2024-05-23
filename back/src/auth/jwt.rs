use std::time::{SystemTime, UNIX_EPOCH};

use base64::{engine::general_purpose, Engine as _};
use jsonwebtoken::{
    decode, encode, errors::Error, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use lazy_static::lazy_static;
use rand::Rng;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref ACCESS_SECRET: String = generate_secret();
    static ref REFRESH_SECRET: String = generate_secret();
}

fn generate_secret() -> String {
    let secret: [u8; 32] = rand::thread_rng().gen();
    general_purpose::STANDARD.encode(&secret)
}

pub fn get_access_secret() -> &'static [u8] {
    ACCESS_SECRET.as_bytes()
}

pub fn get_refresh_secret() -> &'static [u8] {
    REFRESH_SECRET.as_bytes()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    exp: usize,
}

pub fn generate_token(username: &str, secret: &[u8], expiry: usize) -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let iat = since_the_epoch.as_secs();
    let exp = iat + expiry as u64;

    let claims = Claims {
        sub: username.to_owned(),
        exp: exp as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
    .unwrap()
}

pub fn validate_token(token: &str, secret: &[u8]) -> Result<TokenData<Claims>, Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )
}
