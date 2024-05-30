use rocket::serde::Deserialize;
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UserCredentials {
    pub login: String,
    pub password: String,
}
