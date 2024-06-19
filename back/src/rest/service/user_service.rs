use crate::auth::jwt::{generate_token, AccessToken};
use crate::rest::model::error::ErrorResponse;
use crate::rest::model::user::{UserCredentialsDto, UserSigninCredentialsDto};
use entity_manager::pool::DbConn;
use entity_manager::repository::user_repository;

pub async fn signin_user(
    db_conn: DbConn,
    user_credentials: UserSigninCredentialsDto,
) -> AccessToken {
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
) -> Result<(), ErrorResponse> {
    let res = db_conn
        .run(move |c| user_repository::create(c, &user_credentials.into()))
        .await;
    match res {
        Ok(_) => Ok(()),
        Err(db_error) => Err(db_error.into()),
    }
}
