use crate::auth::jwt::{generate_token, AccessToken};
use crate::auth::sha512::convert;
use crate::database::pool::DbConn;
use crate::database::services::user_service::UserService;
use crate::rest::model::UserCredentials;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;

#[post("/login", format = "json", data = "<credentials>")]
pub async fn handle_login(
    db_conn: DbConn,
    credentials: Json<UserCredentials>,
) -> Result<AccessToken, BadRequest<String>> {
    let user_login = credentials.login.clone();
    if db_conn
        .run(move |c| UserService::exists(c, &credentials.login, &convert(&credentials.password)))
        .await
    {
        Ok(generate_token(&user_login))
    } else {
        //TODO: return json error instead of string
        Err(BadRequest("Invalid login or password".to_string()))
    }
}
