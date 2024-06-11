use entity_manager::models::protein::Protein;
use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct ProteinPageInnerDto {
    pub code: String,
}

impl From<Protein> for ProteinPageInnerDto {
    fn from(protein: Protein) -> ProteinPageInnerDto {
        ProteinPageInnerDto { code: protein.code }
    }
}
