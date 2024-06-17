use rocket::serde::Serialize;

//TODO: improve the content ??
#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct JsonErrorMessage {
    pub message: String,
    pub status: rocket::http::Status,
}

impl JsonErrorMessage {
    pub fn new(status: rocket::http::Status, message: String) -> Self {
        JsonErrorMessage { message, status }
    }
}
