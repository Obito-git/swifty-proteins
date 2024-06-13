use crate::auth::jwt::AccessToken;
use crate::rest::model::error::CustomErrorMessage;
use crate::rest::model::user::{UserCredentialsDto, UserDataDto};
use crate::rest::service::user_service::{signin_user, signup_user};
use entity_manager::pool::DbConn;
use rocket::http::Status;
use rocket::response::status::{BadRequest, Custom};
use rocket::serde::json::Json;
use validator::Validate;

#[post("/signin", format = "json", data = "<credentials>")]
pub async fn handle_signin(
    db_conn: DbConn,
    credentials: Json<UserCredentialsDto>,
) -> Result<Json<AccessToken>, BadRequest<String>> {
    //TODO: handle errors
    Ok(Json(signin_user(db_conn, credentials.into_inner()).await))
}

//TODO: try to use the validation guard, for now it's not working and I'm not sure why (infinite loop)
#[post("/signup", format = "json", data = "<credentials>")]
pub async fn handle_signup(
    db_conn: DbConn,
    credentials: Json<UserCredentialsDto>,
) -> Result<Custom<()>, BadRequest<Json<CustomErrorMessage>>> {
    let credentials: UserCredentialsDto = credentials.into_inner();
    match credentials.validate() {
        Ok(_) => match signup_user(db_conn, credentials).await {
            Ok(_) => Ok(Custom(Status::Created, ())),
            Err(e) => Err(BadRequest(Json(e))),
        },
        Err(errors) => {
            let error_messages: Vec<String> = errors
                .field_errors()
                .iter()
                .flat_map(|(_field, errors)| {
                    errors.iter().filter_map(|error| {
                        let message = error.message.clone().unwrap_or_default();
                        if !message.trim().is_empty() {
                            Some(format!("{}", message))
                        } else {
                            None
                        }
                    })
                })
                .collect();
            return Err(BadRequest(Json(CustomErrorMessage::new(
                error_messages.join(", "),
            ))));
        }
    }
}
