#[macro_use] extern crate rocket;

use crate::rest::route::login::{handle_signin, handle_signup};
use database::pool::DbConn;

mod auth;
mod rest;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                //protected_route,
                handle_signin,
                handle_signup,
                //get_proteins_page
            ],
        )
        .attach(DbConn::fairing())
}
