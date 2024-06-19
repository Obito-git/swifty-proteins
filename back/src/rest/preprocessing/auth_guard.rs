use rocket::data::Outcome;
use rocket::data::{self, Data, FromData};
use rocket::http::Status;
use rocket::serde::json::from_str;
use rocket::serde::DeserializeOwned;
use validator::{Validate, ValidationErrors};

use crate::auth::sha512::convert;
use crate::rest::model::error::JsonErrorMessage;
use crate::rest::model::user::{UserCredentialsDto, UserSigninCredentialsDto};
use rocket::data::ByteUnit;
use rocket::request::Request;

async fn deserialize_json<'r, T>(
    req: &'r Request<'_>,
    data: Data<'r>,
) -> Result<T, JsonErrorMessage>
where
    T: DeserializeOwned,
{
    let limit = req
        .rocket()
        .config()
        .limits
        .get("json")
        .unwrap_or(ByteUnit::default());

    let data = data.open(limit).into_string().await.map_err(|_| {
        JsonErrorMessage::new(
            Status::UnprocessableEntity,
            "Failed to read data".to_string(),
        )
    })?;

    let data_string = data.into_inner(); // Convert data to an owned string

    from_str::<T>(&data_string)
        .map_err(|_| JsonErrorMessage::new(Status::UnprocessableEntity, "Invalid JSON".to_string()))
}

#[rocket::async_trait]
impl<'r> FromData<'r> for UserCredentialsDto {
    type Error = JsonErrorMessage;
    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        match deserialize_json::<UserCredentialsDto>(req, data).await {
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
            Err(e) => Outcome::Error((e.status, e)),
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
    type Error = JsonErrorMessage;
    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        match deserialize_json::<UserSigninCredentialsDto>(req, data).await {
            Ok(mut data) => {
                data.password = convert(&data.password);
                Outcome::Success(data)
            },
            Err(e) => Outcome::Error((e.status, e)),
        }
    }
}
