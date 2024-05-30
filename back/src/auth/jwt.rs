use std::time::{SystemTime, UNIX_EPOCH};

use base64::{engine::general_purpose, Engine as _};
use jsonwebtoken::{
    decode, encode, errors::Error, DecodingKey, EncodingKey, Header, TokenData, Validation,
};
use lazy_static::lazy_static;
use rand::Rng;
use rocket::http::ContentType;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket::serde::json::json;
use serde::{Deserialize, Serialize};

//TODO access modifiers
lazy_static! {
    static ref ACCESS_SECRET: String = generate_secret();
}

fn generate_secret() -> String {
    let secret: [u8; 32] = rand::thread_rng().gen();
    general_purpose::STANDARD.encode(&secret)
}

pub fn get_access_secret() -> &'static [u8] {
    ACCESS_SECRET.as_bytes()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    exp: usize,
}

pub struct AccessToken {
    pub token: String,
    pub expiry: u64,
}

//TODO: here to map token to json, find better way to store this code
impl<'r> Responder<'r, 'static> for AccessToken {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        let json = json!({
            "token": self.token,
            "expiry": self.expiry,
        });
        Response::build_from(json.respond_to(req)?)
            .header(ContentType::JSON)
            .ok()
    }
}

const EXPIRY: u64 = 3600;

pub fn generate_token(username: &str) -> AccessToken {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let iat = since_the_epoch.as_secs();
    let exp = iat + EXPIRY;

    let claims = Claims {
        sub: username.to_owned(),
        exp: exp as usize,
    };

    //TODO: handle error
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(get_access_secret()),
    )
    .unwrap();
    AccessToken {
        token,
        expiry: EXPIRY,
    }
}

pub fn validate_token(token: &str, secret: &[u8]) -> Result<TokenData<Claims>, Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default(),
    )
}
