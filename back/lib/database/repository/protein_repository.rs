use crate::models::pagination::{DataPage, Metadata};
use crate::models::protein::Protein;
use crate::schema::proteins as proteins_table;
use diesel::{QueryDsl, RunQueryDsl, SqliteConnection};

const DEFAULT_PAGE_SIZE: i64 = 50;

pub fn read_paginated(
    conn: &mut SqliteConnection,
    page: i64,
    page_size: Option<i64>,
) -> Result<DataPage<Protein>, diesel::result::Error> {
    //TODO: add validation for values, handle panic
    let page_size = page_size.unwrap_or(DEFAULT_PAGE_SIZE);
    let proteins = proteins_table::table
        .offset(page * page_size)
        .limit(page_size)
        .load::<Protein>(conn);
    let total_items = proteins_table::table.count().get_result::<i64>(conn)?;
    let metadata = Metadata {
        total_pages: total_items / page_size,
        current_page: page,
        page_size,
        total_items,
        items_on_page: proteins.as_ref().map(|p| p.len() as i64).unwrap_or(0),
    };
    Ok(DataPage {
        data: proteins?,
        metadata,
    })
}
