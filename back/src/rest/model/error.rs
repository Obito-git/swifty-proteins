use rocket::serde::Serialize;

type RestErrorResponse = (
    rocket::http::Status,
    rocket::serde::json::Json<JsonErrorMessage>,
);

pub enum ErrorResponse {
    UsernameTaken,
    InternalServerError,
    BadRequest(Option<String>),
}

impl ErrorResponse {
    pub fn status(&self) -> rocket::http::Status {
        match self {
            ErrorResponse::UsernameTaken => rocket::http::Status::Conflict,
            ErrorResponse::InternalServerError => rocket::http::Status::InternalServerError,
            ErrorResponse::BadRequest(_) => rocket::http::Status::BadRequest,
        }
    }

    pub fn message(&self) -> String {
        match self {
            ErrorResponse::UsernameTaken => "Username is already taken".to_string(),
            ErrorResponse::InternalServerError => "Internal server error".to_string(),
            ErrorResponse::BadRequest(msg) => msg.clone().unwrap_or("Bad request".to_string()),
        }
    }
}

impl From<ErrorResponse> for RestErrorResponse {
    fn from(entity: ErrorResponse) -> Self {
        let status = entity.status();
        let message = JsonErrorMessage::new(status, entity.message());
        (status, rocket::serde::json::Json(message))
    }
}

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
