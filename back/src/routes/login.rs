use crate::auth::jwt::{generate_token, get_access_secret};

#[get("/hello/<name>/<age>")]
pub fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/")]
pub fn hello_raw() -> String {
    format!(
        "Login mock, your JWT of username Axel: {}",
        generate_token("Axel", get_access_secret(), 900)
    )
}
