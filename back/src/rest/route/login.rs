use crate::auth::jwt::AccessToken;
use crate::rest::model::user::{UserCredentialsDto, UserDataDto};
use crate::rest::service::user_service::{signin_user, signup_user};
use entity_manager::pool::DbConn;
use rocket::http::Status;
use rocket::response::status::{BadRequest, Custom};
use rocket::serde::json::Json;

#[post("/signin", format = "json", data = "<credentials>")]
pub async fn handle_signin(
    db_conn: DbConn,
    credentials: Json<UserCredentialsDto>,
) -> Result<Json<AccessToken>, BadRequest<String>> {
    //TODO: handle errors
    Ok(Json(signin_user(db_conn, credentials.into()).await))
}

#[post("/signup", format = "json", data = "<credentials>")]
pub async fn handle_signup(
    db_conn: DbConn,
    credentials: Json<UserCredentialsDto>,
) -> Result<Custom<Json<UserDataDto>>, BadRequest<String>> {
    let user = signup_user(db_conn, credentials.into()).await;

    Ok(Custom(Status::Created, Json(user)))
}
