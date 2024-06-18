use crate::models::error::DatabaseError;
use crate::models::user::{User, UserCredentials, UserData};
use crate::schema::users as users_table;
use crate::schema::users::{password, username};
use diesel::prelude::*;
use diesel::result::DatabaseErrorInformation;
use diesel::result::Error::DatabaseError as DieselDatabaseError;
use diesel::{RunQueryDsl, SqliteConnection};

//TODO: merge filters
//TODO: test expects

pub fn create(
    connection: &mut SqliteConnection,
    user_data: &UserCredentials,
) -> Result<(), DatabaseError> {
    match diesel::insert_into(users_table::table)
        .values(user_data)
        .execute(connection)
    {
        Ok(_) => Ok(()),
        Err(e) => {
            if let DieselDatabaseError(diesel::result::DatabaseErrorKind::UniqueViolation, _) = e {
                Err(DatabaseError::UniqueViolation(e.message().to_string()))
            } else {
                Err(DatabaseError::InternalError)
            }
        }
    }
}

pub fn exists(connection: &mut SqliteConnection, user_credentials: &UserCredentials) -> bool {
    users_table::table
        .filter(username.eq(user_credentials.username.as_str()))
        .filter(password.eq(user_credentials.password.as_str()))
        .first::<User>(connection)
        .is_ok()
}
