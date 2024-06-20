use crate::rest::model::error::JsonErrorMessage;
use crate::rest::model::pagination::DataPageDto;
use crate::rest::model::protein::ProteinPageInnerDto;
use crate::rest::preprocessing::protected_routes_guard::AuthenticatedUser;
use crate::rest::service::protein_service;
use entity_manager::{pool::DbConn, repository::protein_repository::read_paginated};
use rocket::fs::NamedFile;
use rocket::http::ContentType;
use rocket::http::Status;
use rocket::response::status::BadRequest;
use rocket::serde::json::Json;

//TODO: imporove route/replace return type with DTO
//TODO: make protected AuthenticatedUser
#[get("/proteins?<page>&<filter>")]
pub async fn get_proteins_page(
    db_conn: DbConn,
    user: Result<AuthenticatedUser, JsonErrorMessage>,
    page: Option<i64>,
    filter: Option<String>,
) -> Result<Json<DataPageDto<ProteinPageInnerDto>>, (Status, Json<JsonErrorMessage>)> {
    user.map_err(Into::<(Status, Json<JsonErrorMessage>)>::into)?;
    protein_service::get_proteins_page(db_conn, page, filter)
        .await
        .map(Json)
        .map_err(Into::into)
}
//TODO: implement and make protected
#[get("/proteins/<code>")]
pub async fn get_protein_mock(
    db_conn: DbConn,
    user: Result<AuthenticatedUser, JsonErrorMessage>,
    code: &str,
) -> Result<(ContentType, NamedFile), (Status, Json<JsonErrorMessage>)> {
    user.map_err(Into::<(Status, Json<JsonErrorMessage>)>::into)?;
    protein_service::get_protein_model(db_conn, code)
        .await
        .map(|named_file| (ContentType::new("model", "gltf-binary"), named_file))
        .map_err(Into::into)
}
