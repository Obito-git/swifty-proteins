use entity_manager::{
    pool::DbConn,
    repository::{file_repository, protein_repository},
};
use rocket::fs::NamedFile;

//TODO: error handling
pub async fn get_protein_model(db_conn: DbConn, name: &str) -> Result<NamedFile, String> {
    let protein_code = name.to_string();

    match db_conn
        .run(move |c| protein_repository::read_by_code(c, &protein_code))
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
                            Ok(named_file) => Ok(named_file),
                            Err(e) => Err(format!("Error opening file: {}", e)),
                        }
                    }
                    Ok(None) => Err(format!("HERE SHOULD BE INJECTED AXEL METHOD")),
                    Err(e) => Err(format!("Error reading file metadata: {}", e)),
                }
            } else {
                Err("HERE SHOULD BE INJECTED AXEL METHOD".to_string())
            }
        }
        Ok(None) => Err(format!("No protein found with code: {}", name)),
        Err(e) => Err(format!("Error querying protein: {}", e)),
    }
}
