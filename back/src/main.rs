use crate::rest::route::login::{handle_signin, handle_signup};
use database::pool::DbConn;

#[macro_use]
extern crate rocket;

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
