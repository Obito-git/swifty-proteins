use crate::database::services::user_service::UserService;
use crate::routes::{
    login::{hello, hello_raw},
    proteins::protected_route,
};

#[macro_use]
extern crate rocket;

mod auth;
mod database;
mod routes;

#[launch]
fn rocket() -> _ {
    let mut db = database::establish_connection();
    //let users = Map
    //UserService::create(&mut db, "name", "login", "password");
    rocket::build().mount("/", routes![hello, hello_raw, protected_route])
}
