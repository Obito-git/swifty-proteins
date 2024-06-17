use std::borrow::Cow;

use entity_manager::models::user::{UserCredentials, UserData};
use lazy_static::lazy_static;
use regex::Regex;
use rocket::serde::{Deserialize, Serialize};
use validator::ValidationError;
use validator_derive::Validate;

use crate::auth::sha512::convert;

lazy_static! {
    static ref ALPHANUMERIC_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
}

//TODO: fix error messages
#[derive(Debug, Deserialize, Validate)]
#[serde(crate = "rocket::serde")]
pub struct UserCredentialsDto {
    #[validate(length(min = 2, max = 20, message = "min 2, max 20 characters long"))]
    #[validate(custom(function = "validate_credentials"))]
    pub username: String,

    #[validate(length(min = 6, max = 20, message = "min 6, max 20 characters long"))]
    #[validate(custom(function = "validate_credentials"))]
    pub password: String,
}

fn validate_credentials(value: &str) -> Result<(), ValidationError> {
    if !ALPHANUMERIC_REGEX.is_match(value) {
        return Err(
            ValidationError::new("should be alphanumeric with no spaces")
                .with_message(Cow::Borrowed("should be alphanumeric with no spaces")),
        );
    }
    Ok(())
}

//TODO: move hash conversion to a guard if possible
impl From<UserCredentialsDto> for UserCredentials {
    fn from(credentials: UserCredentialsDto) -> Self {
        UserCredentials {
            username: credentials.username,
            password: convert(&credentials.password),
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserDataDto {
    pub username: String,
}

impl From<UserData> for UserDataDto {
    fn from(user: UserData) -> Self {
        UserDataDto {
            username: user.username,
        }
    }
}
