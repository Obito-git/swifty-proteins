use entity_manager::{pool::DbConn, repository::protein_repository};
use rocket::fs::NamedFile;

//TODO: implement
pub async fn get_protein_model(db_conn: DbConn, name: &str) -> NamedFile {
    let aaa: String = name.to_string().clone();
    match db_conn
        .run(move |c| protein_repository::read_by_code(c, &aaa))
        .await
    {
        Ok(protein) => {
            println!("Protein: {:?}", protein);
            //let path = PathBuf::from(protein.path);
            panic!();
            //NamedFile::open(path).await.ok().unwrap()
        }
        Err(e) => {
            println!("Error: {}", e);
            panic!();
            //NamedFile::open(PathBuf::from("")).await.ok().unwrap()
        }
    }
}
