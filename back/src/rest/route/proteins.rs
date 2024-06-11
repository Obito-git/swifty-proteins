use std::path::PathBuf;

use crate::rest::model::pagination::DataPageDto;
use crate::rest::model::protein::ProteinPageInnerDto;
use entity_manager::{pool::DbConn, repository::protein_repository::read_paginated};
use gltf::json::Root;
use gltf::Gltf;
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
pub fn get_protein_mock(code: String) -> Json<Root> {
    println!("Protein code: {}, returning mock data", code);
    let path = PathBuf::from(
        "/home/amyroshn/workspace/projects/swifty-proteins/back/temp_assets/matilda/scene.gltf",
    );
    let a = Gltf::open(path).unwrap();

    Json(a.document.into_json())
}
