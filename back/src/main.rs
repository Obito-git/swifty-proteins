#[macro_use]
extern crate rocket;

use crate::rest::route::login::{handle_signin, handle_signup};
use crate::rest::route::proteins::get_protein_mock;
use crate::rest::route::proteins::get_proteins_page;
use entity_manager::pool::DbConn;

mod auth;
mod rest;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Method, Status};
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        if request.method() == Method::Options {
            response.set_status(Status::NoContent);
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "POST, PATCH, GET, DELETE",
            ));
            response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        }

        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                //protected_route,
                handle_signin,
                handle_signup,
                get_proteins_page,
                get_protein_mock
            ],
        )
        .attach(DbConn::fairing())
        .attach(CORS)
}
