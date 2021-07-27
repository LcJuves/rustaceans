mod mysql_access;
#[macro_use]
extern crate rocket;
extern crate mysql;

use mysql::serde_json::*;
use mysql_access::read;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/items")]
fn items() -> String {
    match read() {
        Ok(ret) => {
            return json!(ret).to_string();
        }
        _ => {
            return "[]".to_string();
        }
    };
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, items])
}
