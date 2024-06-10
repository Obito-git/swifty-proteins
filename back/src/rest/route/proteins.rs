use crate::rest::model::pagination::DataPageDto;
use crate::{auth::auth_guard::AuthenticatedUser, rest::model::protein::ProteinPageInnerDto};
use database::{
    pool::DbConn,
    repository::protein_repository::read_paginated,
};
use rocket::response::status::BadRequest;

//TODO: imporove route/replace return type with DTO
//TODO: make protected AuthenticatedUser
#[get("/proteins?<page>")]
pub async fn get_proteins_page(
    db_conn: DbConn,
    page: i64,
) -> Result<DataPageDto<ProteinPageInnerDto>, BadRequest<String>> {
    match db_conn.run(move |c| read_paginated(c, page, None)).await {
        Ok(proteins) => Ok(proteins.into()),
        Err(e) => Err(BadRequest(format!("Error: {}", e))),
    }
}
