use crate::auth::jwt::{generate_token, AccessToken};
use crate::database::pool::DbConn;
use crate::database::services::user_service::UserService;
use crate::rest::model::LoginData;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;
#[post("/login", format = "json", data = "<login>")]
pub async fn handle_login(
    db_conn: DbConn,
    login: Json<LoginData>,
) -> Result<AccessToken, BadRequest<String>> {
    let user_login = login.login.clone();
    if db_conn
        .run(move |c| UserService::exists(c, &login.login, &login.password))
        .await
    {
        Ok(generate_token(&user_login))
    } else {
        //TODO: return json error instead of string
        Err(BadRequest("Invalid login or password".to_string()))
    }
}
