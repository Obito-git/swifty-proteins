use crate::models::file::FileMetadata;
use crate::schema::proteins;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Associations, Debug)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(FileMetadata))]
#[diesel(table_name = proteins)]
#[allow(dead_code)]
pub struct Protein {
    pub code: String,
    pub file_metadata_id: Option<i32>,
}
