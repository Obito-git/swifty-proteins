use crate::auth::jwt::AccessToken;
use crate::rest::model::error::JsonErrorMessage;
use crate::rest::model::user::UserCredentialsDto;
use crate::rest::model::user::UserSigninCredentialsDto;
use crate::rest::service::user_service::{signin_user, signup_user};
use entity_manager::pool::DbConn;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;

#[post("/signin", format = "json", data = "<credentials>")]
pub async fn handle_signin(
    db_conn: DbConn,
    credentials: UserSigninCredentialsDto,
) -> Result<Json<AccessToken>, (rocket::http::Status, Json<JsonErrorMessage>)> {
    signin_user(db_conn, credentials)
        .await
        .map(|token| Json(token))
        .map_err(Into::into)
}

#[post("/signup", format = "json", data = "<credentials>")]
pub async fn handle_signup(
    db_conn: DbConn,
    credentials: Result<UserCredentialsDto, JsonErrorMessage>,
) -> Result<Custom<()>, (rocket::http::Status, Json<JsonErrorMessage>)> {
    signup_user(db_conn, credentials?)
        .await
        .map(|_| Custom(Status::Created, ()))
        .map_err(Into::into)
}
