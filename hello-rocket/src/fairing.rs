/**
 * Created at 2021/7/28 22:00
 *
 * @author Liangcheng Juves
 */
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{ContentType, Method};
use rocket::{Request, Response};

pub struct CommonFairing;

#[allow(dead_code)]
impl CommonFairing {
    #[cfg(debug_assertions)]
    const ACCESS_CONTROL_ALLOW_ORIGIN: &'static str = "*";

    #[cfg(not(debug_assertions))]
    const ACCESS_CONTROL_ALLOW_ORIGIN: &'static str = "https://docs.lcjuves.com";
}

#[rocket::async_trait]
impl Fairing for CommonFairing {
    fn info(&self) -> Info {
        Info { name: "CommonFairing", kind: Kind::Request | Kind::Response }
    }

    async fn on_response<'r>(&self, req: &'r Request<'_>, resp: &mut Response<'r>) {
        if req.method() == Method::Get && req.uri().path() == "/items" {
            resp.adjoin_raw_header("Access-Control-Allow-Methods", "GET");
            resp.adjoin_raw_header(
                "Access-Control-Allow-Origin",
                Self::ACCESS_CONTROL_ALLOW_ORIGIN,
            );
            let cty = ContentType::JSON;
            let cty = ContentType::new(cty.top().to_string(), cty.sub().to_string())
                .with_params([("charset", "utf-8")]);
            resp.set_header(cty);
        }
    }
}
