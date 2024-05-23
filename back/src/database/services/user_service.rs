use diesel::{PgConnection, RunQueryDsl, SelectableHelper};
use crate::database::models::user::{NewUser, User};
use crate::database::schema::users;

//TODO: consider to put connection in this struct
pub struct UserService;

impl UserService {
    //TODO: consider to return Result
    pub fn create(connection: &mut PgConnection, name: &str, login: &str, password: &str) -> User {
        let new_user = NewUser {
            name,
            login,
            password,
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(connection)
            //TODO: consider to return Result
            .expect("Error saving new user")
    }
}
