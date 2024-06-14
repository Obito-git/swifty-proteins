use rocket::data::Outcome;
use rocket::data::{self, Data, FromData};
use rocket::http::Status;
use rocket::serde::json::from_str;
use validator::Validate;

use crate::rest::model::error::JsonErrorMessage;
use crate::rest::model::user::UserCredentialsDto;
use rocket::data::ByteUnit;
use rocket::request::Request;

#[rocket::async_trait]
impl<'r> FromData<'r> for UserCredentialsDto {
    type Error = JsonErrorMessage;
    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        let limit = req
            .rocket()
            .config()
            .limits
            .get("json")
            .unwrap_or(ByteUnit::default());
        let data = data.open(limit).into_string().await;

        match data {
            Ok(string) => match from_str::<UserCredentialsDto>(&string) {
                Ok(credentials) => match credentials.validate() {
                    //TODO: add password hashing and inject db connection to check is user exists
                    Ok(_) => Outcome::Success(credentials),
                    Err(errors) => {
                        let error_messages: Vec<String> = errors
                            .field_errors()
                            .iter()
                            .flat_map(|(field, errors)| {
                                errors.iter().filter_map(move |error| {
                                    let message = error.message.clone().unwrap_or_default();
                                    if !message.trim().is_empty() {
                                        Some(format!("{}: {}", field, message))
                                    } else {
                                        None
                                    }
                                })
                            })
                            .collect();
                        Outcome::Error((
                            Status::BadRequest,
                            JsonErrorMessage::new(Status::BadRequest, error_messages.join(", ")),
                        ))
                    }
                },
                //TODO: log it
                Err(_) => Outcome::Error((
                    Status::UnprocessableEntity,
                    JsonErrorMessage::new(Status::UnprocessableEntity, "Invalid JSON".to_string()),
                )),
            },
            Err(_) => Outcome::Error((
                Status::InternalServerError,
                JsonErrorMessage::new(
                    Status::InternalServerError,
                    "Failed to read data".to_string(),
                ),
            )),
        }
    }
}
