use crate::auth::jwt::{generate_token, AccessToken};
use crate::rest::model::error::{ErrorResponse, JsonErrorMessage};
use crate::rest::model::user::{UserCredentialsDto, UserSigninCredentialsDto};
use entity_manager::pool::DbConn;
use entity_manager::repository::user_repository;

pub async fn signin_user(
    db_conn: DbConn,
    user_credentials: UserSigninCredentialsDto,
) -> Result<AccessToken, ErrorResponse> {
    let user_login = user_credentials.username.clone();
    if db_conn
        .run(move |c| user_repository::exists(c, &user_credentials.into()))
        .await
    {
        Ok(generate_token(&user_login))
    } else {
        Err(ErrorResponse::Unauthorized)
    }
}

pub async fn signup_user(
    db_conn: DbConn,
    user_credentials: UserCredentialsDto,
) -> Result<(), ErrorResponse> {
    let res = db_conn
        .run(move |c| user_repository::create(c, &user_credentials.into()))
        .await;
    match res {
        Ok(_) => Ok(()),
        Err(db_error) => Err(db_error.into()),
    }
}
