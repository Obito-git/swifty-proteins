pub struct Metadata {
    pub total_pages: i64,
    pub current_page: i64,
    pub page_size: i64,
    pub total_items: i64,
}

pub struct DataPage<T> {
    pub metadata: Metadata,
    pub data: Vec<T>,
}
