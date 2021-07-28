mod model;
mod mysql_access;

#[macro_use]
extern crate rocket;
extern crate mysql;

use rocket::http::{ContentType, Method, Status};

use rocket::fairing::{Fairing, Info, Kind};

use mysql::serde_json::*;
use mysql_access::read;

use std::future::Future;
use std::io::Cursor;
use std::pin::Pin;
use std::sync::atomic::{AtomicUsize, Ordering};

use rocket::{Data, Request, Response};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub struct MyFairing;

#[rocket::async_trait]
impl Fairing for MyFairing {
    fn info(&self) -> Info {
        Info {
            name: "MyFairing",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, resp: &mut Response<'r>) {
        if req.method() == Method::Get && req.uri().path() == "/items" {
            resp.adjoin_raw_header("Access-Control-Allow-Methods", "GET");
            resp.adjoin_raw_header(
                "Access-Control-Allow-Origin",
                "docs.liangchengj.com,127.0.0.1,localhost",
            );
        }
    }
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
    rocket::build()
        .mount("/", routes![index, items])
        .attach(MyFairing)
}
