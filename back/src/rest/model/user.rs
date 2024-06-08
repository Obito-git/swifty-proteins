use database::models::user::{UserCredentials, UserData};
use rocket::http::ContentType;
use rocket::response::Responder;
use rocket::serde::json::{json, Json};
use rocket::serde::Deserialize;
use rocket::{Request, Response};

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

impl<'r> Responder<'r, 'static> for UserDataDto {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        let json = json!({
            "username": self.username,
        });
        //TODO: check this expect
        Response::build_from(json.respond_to(req)?)
            .header(ContentType::JSON)
            .ok()
    }
}
