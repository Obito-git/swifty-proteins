use crate::rest::routes::login::handle_login;
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
        .mount("/", routes![protected_route, handle_login])
        .attach(DbConn::fairing())
}
