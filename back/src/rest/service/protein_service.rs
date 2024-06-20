use entity_manager::{
    pool::DbConn,
    repository::{file_repository, protein_repository},
};
use log::info;
use rocket::{form::name, fs::NamedFile};

use crate::rest::model::{
    error::ErrorResponse, pagination::DataPageDto, protein::ProteinPageInnerDto,
};

//TODO: error handling
pub async fn get_proteins_page(
    db_conn: DbConn,
    page: Option<i64>,
    filter: Option<String>,
) -> Result<DataPageDto<ProteinPageInnerDto>, ErrorResponse> {
    db_conn
        .run(move |c| protein_repository::read_paginated(c, page, filter))
        .await
        .map(Into::into)
        .map_err(Into::into)
}

pub async fn get_protein_model(db_conn: DbConn, name: &str) -> Result<NamedFile, ErrorResponse> {
    let name_upper = name.to_uppercase();
    let name = name.to_string();
    match db_conn
        .run(move |c| protein_repository::read_by_code(c, &name))
        .await
    {
        Ok(Some(protein)) => {
            if let Some(file_id) = protein.file_metadata_id {
                match db_conn
                    .run(move |c| file_repository::read_by_id(c, file_id))
                    .await
                {
                    Ok(Some(file_metadata)) => {
                        let file_path = file_metadata.path;
                        match NamedFile::open(&file_path).await {
                            Ok(named_file) => {
                                info!("Successfully opened file: {}", file_path);
                                Ok(named_file)
                            }
                            Err(e) => {
                                error!("Cannot open file at {}: {}", file_path, e);
                                Err(ErrorResponse::InternalServerError)
                            }
                        }
                    }
                    Ok(None) => {
                        error!(
                            "File with ID {} associated with protein {} not found in database.",
                            file_id, protein.code
                        );
                        Err(ErrorResponse::InternalServerError)
                    }
                    Err(e) => {
                        error!(
                            "Database error while reading file with ID {}: {}",
                            file_id, e
                        );
                        Err(ErrorResponse::InternalServerError)
                    }
                }
            } else {
                warn!(
                    "No file associated with protein {}. HERE NEED TO INJECT AXEL METHOD",
                    protein.code
                );
                Err(ErrorResponse::BadRequest(Some(
                    "No file associated with this protein. HERE NEED TO INJECT AXEL METHOD"
                        .to_string(),
                )))
            }
        }
        Ok(None) => {
            info!("Protein with code {} not found.", name_upper);
            Err(ErrorResponse::NotFound(format!(
                "Protein with code: {}",
                name_upper
            )))
        }
        Err(e) => {
            error!(
                "Database error while reading protein with code {}: {}",
                name_upper, e
            );
            Err(ErrorResponse::InternalServerError)
        }
    }
}
