use rocket_sync_db_pools::database;

#[database("postgres_db")]
pub struct DbConn(diesel::PgConnection);
