use diesel::prelude::*;
use diesel::{RunQueryDsl, SqliteConnection};

use crate::database::models::user::{NewUser, User};
use crate::database::schema::users as users_table;
use crate::database::schema::users::dsl::{password, username};
use crate::rest::model::user::{UserCredentials, UserData};

pub struct UserService;

impl UserService {
    //TODO: test expects
    pub fn create(connection: &mut SqliteConnection, user_data: &NewUser) -> UserData {
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
            .filter(username.eq(user_credentials.login.as_str()))
            .filter(password.eq(user_credentials.password.as_str()))
            .first::<User>(connection)
            .is_ok()
    }
}
