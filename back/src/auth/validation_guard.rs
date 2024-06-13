/*
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use validator::Validate;

use crate::rest::model::user::UserCredentialsDto;
#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserCredentialsDto {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.content_type() {
            Some(ct) if ct.is_json() => {
                match request.guard::<UserCredentialsDto>().await {
                    Outcome::Success(credentials) => {
                        if let Err(errors) = credentials.validate() {
                            let error_messages: Vec<String> = errors
                                .field_errors()
                                .iter()
                                .flat_map(|(field, errors)| {
                                    errors.iter().map(move |error| {
                                        format!(
                                            "{}: {}",
                                            field,
                                            error.message.clone().unwrap_or_default()
                                        )
                                    })
                                })
                                .collect();
                            return Outcome::Error((Status::BadRequest, error_messages.join(", ")));
                        } else {
                            return Outcome::Success(credentials);
                        }
                    }
                    Outcome::Error(_) => {
                        return Outcome::Error((
                            Status::BadRequest,
                            "Invalid JSON format".to_string(),
                        ));
                    }
                    Outcome::Forward(_) => {
                        todo!("Handle forward outcomes as needed");
                    }
                }
            }
            _ => {
                return Outcome::Error((
                    Status::UnsupportedMediaType,
                    "Unsupported Media Type".to_string(),
                ));
            }
        }
    }
}
*/