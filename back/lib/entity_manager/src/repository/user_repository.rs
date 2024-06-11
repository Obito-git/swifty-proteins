use crate::models::user::{User, UserCredentials, UserData};
use crate::schema::users as users_table;
use crate::schema::users::{password, username};
use diesel::prelude::*;
use diesel::{RunQueryDsl, SqliteConnection};

//TODO: test expects
pub fn create(connection: &mut SqliteConnection, user_data: &UserCredentials) -> UserData {
    diesel::insert_into(users_table::table)
        .values(user_data)
        .execute(connection)
        .expect("Error saving new user");
    UserData::from(
        users_table::table
            .filter(username.eq(&user_data.username))
            .first::<User>(connection)
            .expect("Error loading user"),
    )
}

pub fn exists(connection: &mut SqliteConnection, user_credentials: &UserCredentials) -> bool {
    users_table::table
        .filter(username.eq(user_credentials.username.as_str()))
        .filter(password.eq(user_credentials.password.as_str()))
        .first::<User>(connection)
        .is_ok()
}
