use database::models::protein::Protein;

pub struct ProteinPageDto {
    pub proteins: Vec<Protein>,
    pub page: i32,
    pub total_pages: i32,
    pub total_proteins: i32,
}

impl ProteinPageDto {
    pub fn from_proteins(proteins: Vec<Protein>, page: i32, total_proteins: i32) -> ProteinPageDto {
        let total_pages = (total_proteins as f32 / proteins.len() as f32).ceil() as i32;
        ProteinPageDto {
            proteins,
            page,
            total_pages,
            total_proteins,
        }
    }
}
