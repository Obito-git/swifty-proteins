use crate::auth::jwt::{generate_token, AccessToken};
use crate::rest::model::error::JsonErrorMessage;
use crate::rest::model::user::UserCredentialsDto;
use entity_manager::pool::DbConn;
use entity_manager::repository::user_repository;
use rocket::http::Status;

pub async fn signin_user(db_conn: DbConn, user_credentials: UserCredentialsDto) -> AccessToken {
    let user_login = user_credentials.username.clone();
    if db_conn
        .run(move |c| user_repository::exists(c, &user_credentials.into()))
        .await
    {
        generate_token(&user_login)
    } else {
        //TODO: handle error
        panic!("Invalid login or password")
    }
}

pub async fn signup_user(
    db_conn: DbConn,
    user_credentials: UserCredentialsDto,
) -> Result<(), JsonErrorMessage> {
    let res = db_conn
        .run(move |c| user_repository::create(c, &user_credentials.into()))
        .await;
    res.map_err(|e| JsonErrorMessage::new(Status::BadRequest, e))
}
