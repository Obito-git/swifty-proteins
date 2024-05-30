use crate::rest::routes::login::{handle_signin, handle_signup};
use crate::rest::routes::proteins::protected_route;
use database::pool::DbConn;

#[macro_use]
extern crate rocket;

mod auth;
mod database;
mod rest;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![protected_route, handle_signin, handle_signup])
        .attach(DbConn::fairing())
}
