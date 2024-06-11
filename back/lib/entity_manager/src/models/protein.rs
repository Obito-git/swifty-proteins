use crate::schema::proteins;
use diesel::{Queryable, Selectable};

#[derive(Queryable, Selectable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = proteins)]
#[allow(dead_code)]
pub struct Protein {
    pub code: String,
    pub file_metadata_id: Option<i32>,
}
