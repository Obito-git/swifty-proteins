pub struct PageMetadata {
    pub total_pages: i64,
    pub current_page: i64,
    pub page_size: i64,
    pub total_items: i64,
    pub items_on_page: i64,
}

pub struct DataPage<T> {
    pub metadata: PageMetadata,
    pub data: Vec<T>,
}
