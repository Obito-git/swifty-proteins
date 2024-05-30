use crate::auth::jwt::{generate_token, AccessToken};
use crate::database::models::user::NewUser;
use crate::database::pool::DbConn;
use crate::database::services::user_service::UserService;
use crate::rest::model::user::{UserCredentials, UserData};
use rocket::http::Status;
use rocket::response::status::{BadRequest, Custom};
use rocket::serde::json::Json;

#[post("/signin", format = "json", data = "<credentials>")]
pub async fn handle_signin(
    db_conn: DbConn,
    credentials: Json<UserCredentials>,
) -> Result<AccessToken, BadRequest<String>> {
    let user_login = credentials.login.clone();
    if db_conn
        .run(move |c| UserService::exists(c, &UserCredentials::from_json(credentials)))
        .await
    {
        Ok(generate_token(&user_login))
    } else {
        //TODO: return json error instead of string
        Err(BadRequest("Invalid login or password".to_string()))
    }
}

#[post("/signup", format = "json", data = "<credentials>")]
pub async fn handle_signup(
    db_conn: DbConn,
    credentials: Json<UserCredentials>,
) -> Result<Custom<UserData>, BadRequest<String>> {
    let new_user = NewUser::from_json(credentials);
    let user = db_conn
        .run(move |c| UserService::create(c, &new_user))
        .await;
    Ok(Custom(Status::Created, user))
}
