use rocket::data::Outcome;
use rocket::data::{self, Data, FromData};
use rocket::http::Status;
use rocket::serde::json::from_str;
use validator::{Validate, ValidationErrors};

use crate::auth::sha512::convert;
use crate::rest::model::error::JsonErrorMessage;
use crate::rest::model::user::{UserCredentialsDto, UserSigninCredentialsDto};
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
                Ok(mut credentials) => match credentials.validate() {
                    Ok(_) => {
                        credentials.password = convert(&credentials.password);
                        Outcome::Success(credentials)
                    }
                    Err(errors) => Outcome::Error((
                        Status::BadRequest,
                        JsonErrorMessage::new(Status::BadRequest, parse_error_messages(errors)),
                    )),
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

fn parse_error_messages(errors: ValidationErrors) -> String {
    errors
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
        .collect::<Vec<String>>()
        .join(", ")
}

#[rocket::async_trait]
impl<'r> FromData<'r> for UserSigninCredentialsDto {
    type Error = (Status, String);
    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        let credentials = match UserSigninCredentialsDto::from_data(req, data).await {
            Outcome::Success(mut credentials) => {
                credentials.password = convert(&credentials.password);
                credentials
            }
            Outcome::Forward(e) => return Outcome::Forward(e),
            Outcome::Error((status, e)) => return Outcome::Error((status, e)),
        };
        Outcome::Success(credentials)
    }
}
