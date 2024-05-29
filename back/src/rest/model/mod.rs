use rocket::serde::Deserialize;
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginData {
    pub login: String,
    pub password: String,
}
