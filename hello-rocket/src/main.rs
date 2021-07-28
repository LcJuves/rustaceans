mod model;
mod mysql_access;

#[macro_use]
extern crate rocket;
extern crate mysql;

// use std::io::Cursor;

// use rocket::http::{ContentType, Status};
// use rocket::response::{self, Response, Responder};

use mysql::serde_json::*;
use mysql_access::read;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/items")]
fn items() -> String {
    let json = match read() {
        Ok(ret) => json!(ret).to_string(),
        _ => "[]".to_string(),
    };

    // let mut resp = Response::new();
    // resp.set_header(ContentType::JSON);
    // resp.adjoin_raw_header("Access-Control-Allow-Methods", "GET");
    // resp.adjoin_raw_header(
    //     "Access-Control-Allow-Origin",
    //     "docs.liangchengj.com,127.0.0.1,localhost",
    // );

    // Response::build()
    //     .status(Status::Ok)
    //     .header(ContentType::JSON)
    //     .sized_body(json.len(), Cursor::new(&json))
    //     .ok()

    json
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, items])
}
