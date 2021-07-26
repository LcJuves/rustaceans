mod mysql_access;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/items")]
fn items() -> &'static str {
    "{}"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, items])
}
