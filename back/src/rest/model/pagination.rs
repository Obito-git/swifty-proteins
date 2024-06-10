use database::models::pagination::{DataPage, Metadata};
use rocket::http::ContentType;
use rocket::response::Responder;
use rocket::serde::json::json;
use rocket::serde::Serialize;
use rocket::{Request, Response};

#[derive(Serialize)]
pub struct MetadataDto {
    pub total_pages: i64,
    pub current_page: i64,
    pub page_size: i64,
    pub total_items: i64,
    pub items_on_page: i64,
}

impl From<Metadata> for MetadataDto {
    fn from(metadata: Metadata) -> MetadataDto {
        MetadataDto {
            total_pages: metadata.total_pages,
            current_page: metadata.current_page,
            page_size: metadata.page_size,
            total_items: metadata.total_items,
            items_on_page: metadata.items_on_page,

        }
    }
}

#[derive(Serialize)]
pub struct DataPageDto<T> where T: Serialize {
    pub metadata: MetadataDto,
    pub data: Vec<T>,
}

impl<T, U> From<DataPage<T>> for DataPageDto<U>
where
    U: From<T> + rocket::serde::Serialize,
{
    fn from(data_page: DataPage<T>) -> DataPageDto<U> {
        DataPageDto {
            metadata: MetadataDto::from(data_page.metadata),
            data: data_page.data.into_iter().map(U::from).collect(),
        }
    }
}

impl<'r, T> Responder<'r, 'static> for DataPageDto<T> where T: Serialize {
    fn respond_to(self, req: &'r Request<'_>) -> rocket::response::Result<'static> {
        let json = json!({
            "metadata": self.metadata,
            "data": self.data,
        });
        //TODO: check this expect
        Response::build_from(json.respond_to(req)?)
            .header(ContentType::JSON)
            .ok()
    }
}


