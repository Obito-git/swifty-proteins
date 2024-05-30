use crate::auth::sha512::convert;
use crate::database::models::user::User;
use rocket::http::ContentType;
use rocket::response::Responder;
use rocket::serde::json::{json, Json};
use rocket::serde::Deserialize;
use rocket::{Request, Response};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserCredentials {
    pub login: String,
    pub password: String,
}

impl UserCredentials {
    pub fn from_json(json: Json<UserCredentials>) -> UserCredentials {
        UserCredentials {
            login: json.login.clone(),
            password: convert(&json.password),
        }
    }
}

pub struct UserData {
    pub username: String,
}

impl UserData {
    pub fn from(user: User) -> UserData {
        UserData {
            username: user.username,
        }
    }
}

impl<'r> Responder<'r, 'static> for UserData {
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
