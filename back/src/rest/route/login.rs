use crate::auth::jwt::AccessToken;
use crate::rest::model::error::JsonErrorMessage;
use crate::rest::model::user::UserCredentialsDto;
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
    Ok(Json(signin_user(db_conn, credentials.into_inner()).await))
}

//TODO: test errors
#[post("/signup", format = "json", data = "<credentials>")]
pub async fn handle_signup(
    db_conn: DbConn,
    credentials: Result<UserCredentialsDto, JsonErrorMessage>,
) -> Result<Custom<()>, (rocket::http::Status, Json<JsonErrorMessage>)> {
    match credentials {
        Ok(credentials_dto) => match signup_user(db_conn, credentials_dto).await {
            Ok(_) => Ok(Custom(Status::Created, ())),
            Err(e) => Err((e.status, Json(e))),
        },
        Err(e) => Err((e.status, Json(e))),
    }
}
