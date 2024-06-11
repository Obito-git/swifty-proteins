#[macro_use] extern crate rocket;

use crate::rest::route::proteins::get_proteins_page;
use crate::rest::route::login::{handle_signin, handle_signup};
use entity_manager::pool::DbConn;

use local_storage::hello;

mod auth;
mod rest;

#[launch]
fn rocket() -> _ {
    for _ in 0..10 {
        hello();
    }
    rocket::build()
        .mount(
            "/",
            routes![
                //protected_route,
                handle_signin,
                handle_signup,
                get_proteins_page
            ],
        )
        .attach(DbConn::fairing())
}
