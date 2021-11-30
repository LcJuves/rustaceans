use std::time::{SystemTime, UNIX_EPOCH};

use hyper::body::Buf;
use hyper::{Body, Client, Method, Request, Version};
use hyper_tls::HttpsConnector;
use serde::Deserialize;

use lazy_static::lazy_static;

lazy_static! {
    static ref UA: &'static str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36";
    static ref RID_PWD: &'static str = "32663266326832723286329432963316333833543372337634223430346435003538357036043640365436943760378038663882396439684046411841844252";
}

pub(crate) async fn get_login_redirect_url_and_session_id(
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let client = Client::new();

    let req = Request::builder()
        .header("user-agent", UA.to_string())
        // .header("host", "199.200.0.8")
        // .header("connection", "keep-alive")
        // .header("upgrade-insecure-requests", "1")
        // .header("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
        // .header("referer", "http://199.200.0.8/")
        // .header("accept-encoding", "gzip, deflate")
        // .header("accept-language", "zh-CN,zh;q=0.9")
        .method(Method::GET)
        .uri("http://199.200.0.8/api/v1/login/")
        .version(Version::HTTP_11)
        .body(Body::empty())
        .unwrap();

    let resp = client.request(req).await?;

    println!("resp.headers >>> {:?}", resp.headers());

    let location = String::from_utf8_lossy(resp.headers()["location"].as_bytes()).to_string();

    let session_id = String::from_utf8_lossy(resp.headers()["set-cookie"].as_bytes()).to_string();
    let session_id = (&session_id
        [(session_id.find("=").unwrap() + 1)..(session_id.find(";").unwrap())])
        .to_string();

    Ok((location, session_id))
}

pub(crate) async fn get_authorize_redirect_url_and_session_id(
    url: &str,
    session_id: &str,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let client = Client::new();

    let req = Request::builder()
        .header("user-agent", UA.to_string())
        .header("cookie", format!("sessionid={}", session_id))
        // .header("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
        // .header("accept-encoding", "gzip, deflate")
        // .header("accept-language", "zh-CN,zh;q=0.9")
        // .header("connection", "keep-alive")
        // .header("host", "200.200.0.33")
        // .header("referer", "http://199.200.0.8/")
        // .header("upgrade-insecure-requests", "1")
        .method(Method::GET)
        .uri(url)
        .version(Version::HTTP_11)
        .body(Body::empty())
        .unwrap();

    let resp = client.request(req).await?;

    println!("resp.headers >>> {:?}", resp.headers());

    let location = String::from_utf8_lossy(resp.headers()["location"].as_bytes()).to_string();

    let session_id = String::from_utf8_lossy(resp.headers()["set-cookie"].as_bytes()).to_string();
    let session_id = (&session_id
        [(session_id.find("=").unwrap() + 1)..(session_id.find(";").unwrap())])
        .to_string();

    Ok((location, session_id))
}

pub(crate) async fn get_sign_in_client_id_and_response_type(
    url: &str,
    sso_provider_session: &str,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let client = Client::new();
    let req = Request::builder()
        .header("user-agent", UA.to_string())
        .header(
            "cookie",
            format!("_sso_provider_session={}", sso_provider_session),
        )
        .method(Method::GET)
        .uri(url)
        .version(Version::HTTP_11)
        .body(Body::empty())
        .unwrap();

    let resp = client.request(req).await?;

    println!("resp.headers >>> {:?}", resp.headers());

    let location = String::from_utf8_lossy(resp.headers()["location"].as_bytes()).to_string();

    let client_id =
        (&location[(location.find("=").unwrap() + 1)..(location.find("&").unwrap())]).to_string();

    let response_type = (&location[(location.rfind("=").unwrap() + 1)..]).to_string();

    Ok((client_id, response_type))
}

pub(crate) async fn get_userauth_should_tokens_and_session_id(
    client_id: &str,
    response_type: &str,
) -> Result<(String, String, String), Box<dyn std::error::Error>> {
    let mut url = String::from(
        "https://idtrust.sangfor.com:444/ac_portal/userauth.html?template=default&client_id=",
    );
    url.push_str(client_id);
    url.push_str("&response_type=");
    url.push_str(response_type);
    url.push_str("&auth=oauth");

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let req = Request::builder()
        .header("user-agent", UA.to_string())
        .method(Method::GET)
        .uri(url)
        .version(Version::HTTP_11)
        .body(Body::empty())
        .unwrap();

    let resp = client.request(req).await?;

    println!("resp.headers >>> {:?}", resp.headers());

    let location = String::from_utf8_lossy(resp.headers()["location"].as_bytes()).to_string();

    let auth_token_prefix = "authToken=";
    let app_token_prefix = "appToken=";

    let auth_token = (&location[(location.find(auth_token_prefix).unwrap()
        + auth_token_prefix.len())
        ..(location.find(&format!("&{}", app_token_prefix)).unwrap())])
        .to_string();

    let app_token = (&location[(location.find(app_token_prefix).unwrap() + app_token_prefix.len())
        ..(location.find("&auth=").unwrap())])
        .to_string();

    let session_id = String::from_utf8_lossy(resp.headers()["set-cookie"].as_bytes()).to_string();
    let session_id = (&session_id
        [(session_id.find("=").unwrap() + 1)..(session_id.find(";").unwrap())])
        .to_string();

    Ok((auth_token, app_token, session_id))
}

pub(crate) async fn send_sms_and_get_user_name(
    auth_token: &str,
    app_token: &str,
    session_id: &str,
    tel: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut body = String::from("authToken=");
    body.push_str(auth_token);
    body.push_str("&appToken=");
    body.push_str(app_token);
    body.push_str(
        "&auth=oauth&template=default&uuid=014a6560486429cada00afc53fe1017c&opr=getSmsCode&phone=",
    );
    body.push_str(tel);

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let req = Request::builder()
        .header("user-agent", UA.to_string())
        .header("Cookie", format!("AUTHSESSID={}", session_id))
        .header("Zxy", "")
        .method(Method::POST)
        .uri("https://idtrust.sangfor.com:444/ac_portal/login.php")
        .version(Version::HTTP_11)
        .body(Body::from(body))
        .unwrap();

    let resp = client.request(req).await?;

    println!("resp.headers >>> {:?}", resp.headers());

    Ok(())
}

pub(crate) async fn get_handshake(
    session_id: &str,
) -> Result<(Option<String>, String), Box<dyn std::error::Error>> {
    let time_millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
        .to_string();

    let mut body = String::from("rid=");
    body.push_str(&time_millis);

    // println!("body >>> {}", &body);

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let req = Request::builder()
        .header("user-agent", UA.to_string())
        .header("Cookie", format!("AUTHSESSID={}", session_id))
        .header(
            "Content-Type",
            "application/x-www-form-urlencoded; charset=UTF-8",
        )
        .method(Method::POST)
        .uri("https://idtrust.sangfor.com:444/portal/handshake")
        .version(Version::HTTP_11)
        .body(Body::from(body))
        .unwrap();

    let resp = client.request(req).await?;

    println!("resp.headers >>> {:?}", resp.headers());
    // println!();

    let resp_body = hyper::body::aggregate(resp).await?;
    let rid_resp: RidResp = serde_json::from_reader(resp_body.reader())?;

    // println!("rid_resp >>> {:?}", rid_resp);
    // println!();

    if rid_resp.success {
        Ok((Some(rid_resp.rid), time_millis))
    } else {
        Ok((None, time_millis))
    }
}

#[derive(Deserialize, Debug)]
struct RidResp {
    rid: String,
    #[allow(unused)]
    success: bool,
}
