use diesel::prelude::*;
use diesel::{PgConnection, RunQueryDsl};

use crate::database::models::user::User;
use crate::database::schema::users as users_table;
use crate::database::schema::users::dsl::{password, username};

pub struct UserService;

impl UserService {
    //TODO: consider to return Result and better args names
    //TODO: apply hash to password

    /*
    pub fn create(
        connection: &mut PgConnection,
        name: &str,
        new_login: &str,
        new_password: &str,
    ) -> User {
        let new_user = NewUser {
            name,
            login: new_login,
            password: new_password,
        };

        diesel::insert_into(users_table::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(connection)
            //TODO: consider to return Result
            .expect("Error saving new user")
    }

     */

    pub fn exists(
        connection: &mut PgConnection,
        search_username: &str,
        search_password: &str,
    ) -> bool {
        //TODO: apply hash to password
        users_table::table
            .filter(username.eq(search_username))
            .filter(password.eq(search_password))
            .first::<User>(connection)
            .is_ok()
    }
}
