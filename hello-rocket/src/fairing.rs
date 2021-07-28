/**
 * Created at 2021/7/28 22:00
 *
 * @author Liangcheng Juves
 */
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Method;
use rocket::{Request, Response};

pub struct CommomFairing;

#[rocket::async_trait]
impl Fairing for CommomFairing {
    fn info(&self) -> Info {
        Info {
            name: "CommomFairing",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, resp: &mut Response<'r>) {
        if req.method() == Method::Get && req.uri().path() == "/items" {
            resp.adjoin_raw_header("Access-Control-Allow-Methods", "GET");
            resp.adjoin_raw_header("Access-Control-Allow-Origin", "*");
        }
    }
}
