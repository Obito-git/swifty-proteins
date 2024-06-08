use crate::auth::jwt::{generate_token, AccessToken};
use crate::rest::model::user::{UserCredentialsDto, UserDataDto};
use crate::rest::service::user_service::{signin_user, signup_user};
use database::models::user::UserCredentials;
use database::pool::DbConn;
use rocket::http::Status;
use rocket::response::status::{BadRequest, Custom};
use rocket::serde::json::Json;

#[post("/signin", format = "json", data = "<credentials>")]
pub async fn handle_signin(
    db_conn: DbConn,
    credentials: Json<UserCredentialsDto>,
) -> Result<AccessToken, BadRequest<String>> {
    //TODO: handle errors
    Ok(signin_user(db_conn, credentials.into()).await)
}

#[post("/signup", format = "json", data = "<credentials>")]
pub async fn handle_signup(
    db_conn: DbConn,
    credentials: Json<UserCredentialsDto>,
) -> Result<Custom<UserDataDto>, BadRequest<String>> {
    let user = signup_user(db_conn, credentials.into()).await;

    Ok(Custom(Status::Created, user))
}
