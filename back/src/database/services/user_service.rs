use diesel::prelude::*;
use diesel::{PgConnection, RunQueryDsl};

use crate::database::models::user::{NewUser, User};
use crate::database::schema::users as users_table;
use crate::database::schema::users::dsl::{password, username};
use crate::rest::model::user::{UserCredentials, UserData};

pub struct UserService;

impl UserService {
    //TODO: consider to return Result
    //TODO: handle already existing username

    pub fn create(connection: &mut PgConnection, user_data: &NewUser) -> UserData {
        let new_user = diesel::insert_into(users_table::table)
            .values(user_data)
            .returning(User::as_returning())
            .get_result(connection)
            //TODO: consider to return Result
            .expect("Error saving new user");
        UserData::from(new_user)
    }

    pub fn exists(connection: &mut PgConnection, user_credentials: &UserCredentials) -> bool {
        users_table::table
            .filter(username.eq(user_credentials.login.as_str()))
            .filter(password.eq(user_credentials.password.as_str()))
            .first::<User>(connection)
            .is_ok()
    }
}
