mod fairing;
mod model;
mod mysql_access;

#[macro_use]
extern crate rocket;
extern crate mysql;

use mysql::serde_json::json;

use fairing::CommomFairing;
use mysql_access::read;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/items")]
fn items() -> Option<String> {
    let json = match read() {
        Ok(ret) => json!(ret).to_string(),
        _ => "[]".to_string(),
    };

    Some(json)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, items])
        .attach(CommomFairing)
}
