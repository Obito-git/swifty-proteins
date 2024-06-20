use crate::models::error::DatabaseError;
use crate::models::pagination::{DataPage, PageMetadata};
use crate::models::protein::Protein;
use crate::schema::proteins as proteins_table;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SqliteConnection, TextExpressionMethods};
use log::error;

const DEFAULT_PAGE_SIZE: i64 = 50;

//TODO: add validation to filter be alphanumeric and page be positive integer and greater than 0
pub fn read_paginated(
    conn: &mut SqliteConnection,
    page: Option<i64>,
    filter: Option<String>,
) -> Result<DataPage<Protein>, DatabaseError> {
    let page = page.unwrap_or(1) - 1;
    let filter_string = format!("%{}%", filter.unwrap_or("".to_string()));

    let proteins_query = proteins_table::table
        .filter(proteins_table::code.like(&filter_string))
        .offset(page * DEFAULT_PAGE_SIZE)
        .limit(DEFAULT_PAGE_SIZE);

    let proteins = proteins_query.load::<Protein>(conn)?;

    let total_items = proteins_table::table
        .filter(proteins_table::code.like(&filter_string))
        .count()
        .get_result::<i64>(conn)?;

    let metadata = PageMetadata {
        total_pages: (total_items + DEFAULT_PAGE_SIZE - 1) / DEFAULT_PAGE_SIZE,
        current_page: page + 1,
        page_size: DEFAULT_PAGE_SIZE,
        total_items,
        items_on_page: proteins.len() as i64,
    };

    Ok(DataPage {
        data: proteins,
        metadata,
    })
}

pub fn read_by_code(
    conn: &mut SqliteConnection,
    code: &str,
) -> Result<Option<Protein>, DatabaseError> {
    match proteins_table::table
        .filter(proteins_table::code.eq(code))
        .first::<Protein>(conn)
    {
        Ok(protein) => Ok(Some(protein)),
        Err(diesel::result::Error::NotFound) => Ok(None),
        Err(err) => {
            error!("Error while reading protein by code: {}. Error: {}", code, err);
            Err(err.into())
        }
    }
}
