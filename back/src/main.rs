use crate::database::services::user_service::UserService;
use crate::routes::{login::{hello, hello_raw}, proteins::protected_route};

#[macro_use]
extern crate rocket;

mod auth;
mod routes;
mod database;

#[launch]
fn rocket() -> _ {
    //let users = Map
    let mut db = database::establish_connection();
    UserService::create(&mut db, "name", "login", "password");
    rocket::build().mount("/", routes![hello, hello_raw, protected_route])
}
