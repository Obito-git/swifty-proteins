use crate::routes::{login::{hello, hello_raw}, proteins::protected_route};

#[macro_use]
extern crate rocket;

mod auth;
mod routes;

#[launch]
fn rocket() -> _ {
    //let users = Map
    rocket::build().mount("/", routes![hello, hello_raw, protected_route])
}
