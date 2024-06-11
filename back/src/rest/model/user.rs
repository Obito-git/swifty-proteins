use entity_manager::models::user::{UserCredentials, UserData};
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

use crate::auth::sha512::convert;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserCredentialsDto {
    pub username: String,
    pub password: String,
}

impl From<Json<UserCredentialsDto>> for UserCredentialsDto {
    fn from(json: Json<UserCredentialsDto>) -> Self {
        UserCredentialsDto {
            username: json.username.clone(),
            password: convert(&json.password),
        }
    }
}

impl From<UserCredentialsDto> for UserCredentials {
    fn from(credentials: UserCredentialsDto) -> Self {
        UserCredentials {
            username: credentials.username,
            password: credentials.password,
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
