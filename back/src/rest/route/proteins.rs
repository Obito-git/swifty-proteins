use std::path::PathBuf;

use crate::rest::model::pagination::DataPageDto;
use crate::rest::model::protein::ProteinPageInnerDto;
use crate::rest::service::protein_service;
use entity_manager::{pool::DbConn, repository::protein_repository::read_paginated};
use rocket::fs::NamedFile;
use rocket::http::ContentType;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;

//TODO: imporove route/replace return type with DTO
//TODO: make protected AuthenticatedUser
#[get("/proteins?<page>")]
pub async fn get_proteins_page(
    db_conn: DbConn,
    page: i64,
) -> Result<Json<DataPageDto<ProteinPageInnerDto>>, BadRequest<String>> {
    match db_conn.run(move |c| read_paginated(c, page, None)).await {
        Ok(proteins) => Ok(Json(proteins.into())),
        Err(e) => Err(BadRequest(format!("Error: {}", e))),
    }
}
//TODO: implement and make protected
#[get("/proteins/<code>")]
pub async fn get_protein_mock(
    db_conn: DbConn,
    code: &str,
) -> Result<(ContentType, NamedFile), BadRequest<String>> {
    match protein_service::get_protein_model(db_conn, code).await {
        Ok(named_file) => {
            let content_type = ContentType::new("model", "gltf-binary");
            Ok((content_type, named_file))
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            Err(BadRequest(e))
        }
    }
}
