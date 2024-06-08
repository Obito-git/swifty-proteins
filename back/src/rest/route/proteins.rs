use crate::auth::auth_guard::AuthenticatedUser;
/*
use crate::database::models::protein::Protein;
use crate::database::pool::DbConn;
use crate::database::services::protein_service::get_proteins_page1;
use crate::rest::model::protein::ProteinPageDto;
use rocket::response::status::BadRequest;

#[get("/protected")]
pub fn protected_route(user: AuthenticatedUser) -> String {
    format!("Hello, {}!", user.username)
}

//TODO: replace param with optional, bad request with struct
#[get("/proteins?<page>")]
pub async fn get_proteins_page(
    db_conn: DbConn,
    page: i64,
) -> Result<ProteinPageDto, BadRequest<String>> {
    match db_conn
        .run(move |c| get_proteins_page1(c, page, None))
        .await
    {
        Ok(proteins) => Ok(ProteinPageDto::from_proteins(proteins, 1, 1)),
        Err(e) => Err(BadRequest(format!("Error: {}", e))),
    }
}


 */
