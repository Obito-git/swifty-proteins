use crate::auth::jwt::{validate_token, Claims};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use std::str::FromStr;

use super::jwt::get_access_secret;

pub struct AuthenticatedUser {
    pub username: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.len() != 1 {
            return Outcome::Error((Status::Unauthorized, ()));
        }

        let token_str = keys[0];
        if !token_str.starts_with("Bearer ") {
            return Outcome::Error((Status::Unauthorized, ()));
        }

        let token = &token_str[7..];

        match validate_token(token, get_access_secret()) {
            Ok(claims) => Outcome::Success(AuthenticatedUser {
                username: claims.claims.sub,
            }),
            Err(_) => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
