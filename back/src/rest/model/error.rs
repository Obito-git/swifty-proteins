use rocket::serde::Serialize;

//TODO: improve the content ??
#[derive(Debug, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CustomErrorMessage {
    pub message: String,
}

impl CustomErrorMessage {
    pub fn new(message: String) -> Self {
        CustomErrorMessage { message }
    }
}
