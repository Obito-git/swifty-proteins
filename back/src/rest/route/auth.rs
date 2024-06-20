use crate::auth::jwt::AccessToken;
use crate::rest::model::error::JsonErrorMessage;
use crate::rest::model::user::UserCredentialsDto;
use crate::rest::model::user::UserSigninCredentialsDto;
use crate::rest::preprocessing::protected_routes_guard::AuthenticatedUser;
use crate::rest::service::user_service;
use entity_manager::pool::DbConn;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;

#[post("/signin", format = "json", data = "<credentials>")]
pub async fn handle_signin(
    db_conn: DbConn,
    credentials: UserSigninCredentialsDto,
) -> Result<Json<AccessToken>, (Status, Json<JsonErrorMessage>)> {
    user_service::signin_user(db_conn, credentials)
        .await
        .map(|token| Json(token))
        .map_err(Into::into)
}

#[get("/signin")]
pub async fn handle_signin_with_auth(
    user: Result<AuthenticatedUser, JsonErrorMessage>,
) -> Result<Status, (Status, Json<JsonErrorMessage>)> {
    user.map(|_| Status::Ok).map_err(Into::into)
}

#[post("/signup", format = "json", data = "<credentials>")]
pub async fn handle_signup(
    db_conn: DbConn,
    credentials: Result<UserCredentialsDto, JsonErrorMessage>,
) -> Result<Custom<()>, (rocket::http::Status, Json<JsonErrorMessage>)> {
    user_service::signup_user(db_conn, credentials?)
        .await
        .map(|_| Custom(Status::Created, ()))
        .map_err(Into::into)
}
