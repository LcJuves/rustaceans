mod fairing;
mod model;
mod mysql_access;

#[macro_use]
extern crate rocket;
extern crate mysql;

use mysql::serde_json::json;

use fairing::CommonFairing;

use model::items_resp::ItemResp;
use mysql_access::read;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/items")]
fn items() -> Option<String> {
    let items_resp = match read() {
        Ok(ret) => Some(ItemResp::new(1u8, Some(ret))),
        Err(_) => Some(ItemResp::new(0u8, None)),
    };
    let json = json!(items_resp).to_string();

    Some(json)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, items]).attach(CommonFairing)
}
