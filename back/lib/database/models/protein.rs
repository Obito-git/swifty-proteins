use crate::schema::proteins;
use diesel::{Queryable, Selectable};
use serde::Serialize;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = proteins)]
#[allow(dead_code)]
pub struct Protein {
    pub code: String,
}
