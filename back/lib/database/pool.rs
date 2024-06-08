use rocket_sync_db_pools::database;

#[database("sqlite_db")]
pub struct DbConn(diesel::SqliteConnection);
