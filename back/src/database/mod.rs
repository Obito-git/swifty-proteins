mod models;
mod schema;
pub mod services;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;

pub fn establish_connection() -> PgConnection {
    for _ in 1..10 {
        print!(
            "{}",
            env::var("DATABASE_URL").expect("DATABASE_URL must be set")
        );
    }
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
