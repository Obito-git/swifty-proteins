use crate::schema::users;
use diesel::{Insertable, Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = users)]
#[allow(dead_code)]
pub struct User {
    id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserCredentials {
    pub username: String,
    pub password: String,
}

pub struct UserData {
    pub username: String,
}

impl From<User> for UserData {
    fn from(user: User) -> UserData {
        UserData {
            username: user.username,
        }
    }
}
