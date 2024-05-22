#[macro_use] extern crate rocket;

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[get("/")]
fn hello_raw() -> String {
    format!("Hello world!")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, hello_raw])
}
