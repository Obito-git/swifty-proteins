use crate::models::user::{User, UserCredentials, UserData};
use crate::schema::users as users_table;
use crate::schema::users::{password, username};
use diesel::prelude::*;
use diesel::{RunQueryDsl, SqliteConnection};

//TODO: merge filters
//TODO: test expects

pub fn create(
    connection: &mut SqliteConnection,
    user_data: &UserCredentials,
) -> Result<(), String> {
    match diesel::insert_into(users_table::table)
        .values(user_data)
        .execute(connection)
    {
        Ok(_) => Ok(()),
        //TODO: improve error handling (log error and be sure to return a proper error message)
        Err(_) => Err(format!("username {} already exists", user_data.username)),
    }
}

pub fn exists(connection: &mut SqliteConnection, user_credentials: &UserCredentials) -> bool {
    users_table::table
        .filter(username.eq(user_credentials.username.as_str()))
        .filter(password.eq(user_credentials.password.as_str()))
        .first::<User>(connection)
        .is_ok()
}

pub fn exists_by_username(connection: &mut SqliteConnection, other_username: &str) -> bool {
    users_table::table
        .filter(username.eq(other_username))
        .first::<User>(connection)
        .is_ok()
}
