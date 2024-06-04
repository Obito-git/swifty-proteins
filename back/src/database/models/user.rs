use crate::auth::sha512::convert;
use crate::database::schema::users;
use crate::rest::model::user::UserCredentials;
use diesel::prelude::*;
use rocket::serde::json::Json;

#[derive(Queryable, Selectable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = users)]
#[allow(dead_code)]
pub struct User {
    id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

impl NewUser {
    pub fn from_json(json: Json<UserCredentials>) -> NewUser {
        NewUser {
            username: json.login.clone(),
            password: convert(&json.password),
        }
    }
}
