use crate::schema::file_metadata as files;
use diesel::prelude::*;

#[derive(Queryable, Identifiable, Selectable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(table_name = files)]
pub struct FileMetadata {
    pub id: i32,
    pub name: String,
    pub path: String,
}
