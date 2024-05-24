use crate::database::models::user::{NewUser, User};
use crate::database::schema::users as users_table;
use crate::database::schema::users::dsl::{login, password, users};
use diesel::prelude::*;
use diesel::{PgConnection, RunQueryDsl, SelectableHelper};

//TODO: consider to put connection in this struct
pub struct UserService;

impl UserService {
    //TODO: consider to return Result and better args names
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

    pub fn exists(
        connection: &mut PgConnection,
        search_login: &str,
        search_password: &str,
    ) -> bool {
        users_table::table
            .filter(login.eq(search_login))
            .filter(password.eq(search_password))
            .first::<User>(connection)
            .is_ok()
    }
}
