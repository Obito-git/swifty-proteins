use crate::{
    models::{error::DatabaseError, file::FileMetadata},
    schema::file_metadata,
};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection};

pub fn read_by_id(
    conn: &mut SqliteConnection,
    id: i32,
) -> Result<Option<FileMetadata>, DatabaseError> {
    match file_metadata::table
        .filter(file_metadata::id.eq(id))
        .first::<FileMetadata>(conn)
    {
        Ok(file) => Ok(Some(file)),
        Err(diesel::result::Error::NotFound) => Ok(None),
        Err(err) => Err(err.into()),
    }
}
