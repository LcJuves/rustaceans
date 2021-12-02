use std::io::{stdin, stdout, BufRead, Write};

use guid_create::GUID;
use josekit::jws::{JwsHeader, HS256};
use josekit::jwt::{encode_with_signer, JwtPayload};
use josekit::{JoseError, Value};

use hyper::body::Buf;
use hyper::{Body, Client, Method, Request, Version};
use hyper_tls::HttpsConnector;

use lazy_static::lazy_static;

lazy_static! {
    static ref UA: &'static str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36";
    static ref JWT_KEY: &'static str = "32293231323532373241325132713273328533033339335733613403343934413469350335713605364136513735376937813863393139494017409141594161";
}

pub(crate) async fn req_api_v1_login() -> Result<(String, String), Box<dyn std::error::Error>> {
    let client = Client::new();

    let req = Request::builder()
        .header("user-agent", UA.to_string())
        .method(Method::GET)
        .uri("http://199.200.0.8/api/v1/login/")
        .version(Version::HTTP_11)
        .body(Body::empty())
        .unwrap();

    let resp = client.request(req).await?;

    println!("resp.headers >>> {:?}", resp.headers());

    let redirect_url = String::from_utf8_lossy(resp.headers()["location"].as_bytes()).to_string();

    let sessionid = String::from_utf8_lossy(resp.headers()["set-cookie"].as_bytes()).to_string();
    let sessionid = (&sessionid
        [(sessionid.find("=").unwrap() + 1)..(sessionid.find(";").unwrap())])
        .to_string();

    Ok((redirect_url, sessionid))
}

pub(crate) async fn req_ss_auth_att_oauth2_authorize(
    url: &str,
    sessionid: &str,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let client = Client::new();

    let req = Request::builder()
        .header("user-agent", UA.to_string())
        .header("cookie", format!("sessionid={}", sessionid))
        .method(Method::GET)
        .uri(url)
        .version(Version::HTTP_11)
        .body(Body::empty())
        .unwrap();

    let resp = client.request(req).await?;

    println!("resp.headers >>> {:?}", resp.headers());

    let redirect_url = String::from_utf8_lossy(resp.headers()["location"].as_bytes()).to_string();

    let sso_provider_session =
        String::from_utf8_lossy(resp.headers()["set-cookie"].as_bytes()).to_string();
    let sso_provider_session = (&sso_provider_session
        [(sso_provider_session.find("=").unwrap() + 1)..(sso_provider_session.find(";").unwrap())])
        .to_string();

    Ok((redirect_url, sso_provider_session))
}

pub(crate) async fn req_ss_users_sign_in(
    url: &str,
    sso_provider_session: &str,
) -> Result<(String, String, String), Box<dyn std::error::Error>> {
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

    let sso_provider_session =
        String::from_utf8_lossy(resp.headers()["set-cookie"].as_bytes()).to_string();
    let sso_provider_session = (&sso_provider_session
        [(sso_provider_session.find("=").unwrap() + 1)..(sso_provider_session.find(";").unwrap())])
        .to_string();

    Ok((client_id, response_type, sso_provider_session))
}

pub(crate) async fn req_ac_portal_userauth(
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

    let authsessid = String::from_utf8_lossy(resp.headers()["set-cookie"].as_bytes()).to_string();
    let authsessid = (&authsessid
        [(authsessid.find("=").unwrap() + 1)..(authsessid.find(";").unwrap())])
        .to_string();

    Ok((auth_token, app_token, authsessid))
}

pub(crate) async fn req_ac_portal_login(
    auth_token: &str,
    app_token: &str,
    authsessid: &str,
    phone_num: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut body = String::from("authToken=");
    body.push_str(auth_token);
    body.push_str("&appToken=");
    body.push_str(app_token);
    body.push_str(
        "&auth=oauth&template=default&uuid=014a6560486429cada00afc53fe1017c&opr=getSmsCode&phone=",
    );
    body.push_str(phone_num);

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let req = Request::builder()
        .header("user-agent", UA.to_string())
        .header("Cookie", format!("AUTHSESSID={}", authsessid))
        .header(
            "Content-Type",
            "application/x-www-form-urlencoded; charset=UTF-8",
        )
        .header("Zxy", jwt_sign_with_guid(&gen_guid(), &JWT_KEY).unwrap())
        .method(Method::POST)
        .uri("https://idtrust.sangfor.com:444/ac_portal/login.php")
        .version(Version::HTTP_11)
        .body(Body::from(body))
        .unwrap();

    let resp = client.request(req).await?;

    println!("resp.headers >>> {:?}", resp.headers());

    let resp_body = hyper::body::aggregate(resp).await?;
    let mut resp_json = Vec::new();
    std::io::copy(&mut resp_body.reader(), &mut resp_json)?;
    let resp_json = String::from_utf8_lossy(&resp_json);
    let resp_json = resp_json.replace("'", r#"""#);
    // println!("resp_json >>> {}", resp_json);
    let resp_json: serde_json::Value = serde_json::from_str(&resp_json)?;

    if let serde_json::Value::Bool(success) = resp_json["success"] {
        if success {
            let user_name = resp_json["userName"].to_string();
            return Ok((&user_name[1..(user_name.len() - 1)]).to_string());
        } else {
            eprintln!("msg >>> {}", resp_json["msg"]);
            std::process::exit(-1);
        }
    }

    std::process::exit(-1);
}

pub(crate) async fn verify_sms_req_ac_portal_login(
    auth_token: &str,
    app_token: &str,
    session_id: &str,
    phone_num: &str,
    user_name: &str,
    sms_code: &u32,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut body = String::from("authToken=");
    body.push_str(auth_token);
    body.push_str("&appToken=");
    body.push_str(app_token);
    body.push_str(
        "&auth=oauth&template=default&uuid=014a6560486429cada00afc53fe1017c&opr=firstSmsLogin&phone=",
    );
    body.push_str(phone_num);
    body.push_str("&smsCode=");
    body.push_str(&sms_code.to_string());
    body.push_str("&userName=");
    body.push_str(user_name);

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let req = Request::builder()
        .header("user-agent", UA.to_string())
        .header("Cookie", format!("AUTHSESSID={}", session_id))
        .header(
            "Content-Type",
            "application/x-www-form-urlencoded; charset=UTF-8",
        )
        .header("Zxy", jwt_sign_with_guid(&gen_guid(), &JWT_KEY).unwrap())
        .method(Method::POST)
        .uri("https://idtrust.sangfor.com:444/ac_portal/login.php")
        .version(Version::HTTP_11)
        .body(Body::from(body))
        .unwrap();

    let resp = client.request(req).await?;

    println!("resp.headers >>> {:?}", resp.headers());

    let resp_body = hyper::body::aggregate(resp).await?;
    let resp_json: serde_json::Value = serde_json::from_reader(resp_body.reader())?;
    // println!("resp_json >>> {}", resp_json);

    if let serde_json::Value::Bool(success) = resp_json["success"] {
        if success {
            if let serde_json::Value::String(location) = &resp_json["location"] {
                return Ok(location.clone());
            }
        } else {
            eprintln!("msg >>> {}", resp_json["msg"]);
            std::process::exit(-1);
        }
    }

    std::process::exit(-1);
}

pub(crate) async fn req_ss_login(
    url: &str,
    sso_provider_session: &str,
) -> Result<String, Box<dyn std::error::Error>> {
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

    let redirect_url = String::from_utf8_lossy(resp.headers()["location"].as_bytes()).to_string();

    Ok(redirect_url)
}

pub(crate) async fn req_api_v1_login_callback(
    url: &str,
    sessionid: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();

    let req = Request::builder()
        .header("user-agent", UA.to_string())
        .header("cookie", format!("sessionid={}", sessionid))
        .method(Method::GET)
        .uri(url)
        .version(Version::HTTP_11)
        .body(Body::empty())
        .unwrap();

    let mut resp = client.request(req).await?;

    println!("resp.headers >>> {:?}", resp.headers());

    use hyper::body::HttpBody as _;
    use tokio::io::{stdout, AsyncWriteExt as _};

    // And now...
    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    }

    let ep_jwt_token_current =
        String::from_utf8_lossy(resp.headers()["set-cookie"].as_bytes()).to_string();
    let ep_jwt_token_current = (&ep_jwt_token_current
        [(ep_jwt_token_current.find("=").unwrap() + 1)..(ep_jwt_token_current.find(";").unwrap())])
        .to_string();

    Ok(ep_jwt_token_current)
}

////////////////////////////////////////////////////////
////////////////////////////////////////////////////////
////////////////////////////////////////////////////////

pub(crate) async fn sign_in_tp_by_sms(
    phone_num: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let (redirect_url, sessionid) = req_api_v1_login().await?;

    let (redirect_url, sso_provider_session) =
        req_ss_auth_att_oauth2_authorize(&redirect_url, &sessionid).await?;

    let (client_id, response_type, sso_provider_session) =
        req_ss_users_sign_in(&redirect_url, &sso_provider_session).await?;

    let (auth_token, app_token, authsessid) =
        req_ac_portal_userauth(&client_id, &response_type).await?;

    let user_name = req_ac_portal_login(&auth_token, &app_token, &authsessid, phone_num).await?;

    let sms_code = read_stdin_sms_code()?;
    let redirect_url = verify_sms_req_ac_portal_login(
        &auth_token,
        &app_token,
        &authsessid,
        phone_num,
        &user_name,
        &sms_code,
    )
    .await?;

    let redirect_url = req_ss_login(&redirect_url, &sso_provider_session).await?;
    let ep_jwt_token_current = req_api_v1_login_callback(&redirect_url, &sessionid).await?;

    Ok(ep_jwt_token_current)
}

////////////////////////////////////////////////////////
////////////////////////////////////////////////////////
////////////////////////////////////////////////////////

pub fn read_stdin_sms_code() -> std::io::Result<u32> {
    let mut sms_code = String::new();

    stdout().write(b"Please enter the SMS verification code you received: ")?;
    stdout().flush()?;
    stdin().lock().read_line(&mut sms_code)?;

    let sms_code = sms_code[..(sms_code
        .rfind("\r")
        .unwrap_or(sms_code.rfind("\n").unwrap()))]
        .to_string();

    println!("sms_code >>> {}", sms_code);

    use std::str::FromStr;
    Ok(u32::from_str(&sms_code).unwrap())
}

pub(crate) fn jwt_sign_with_guid(guid: &str, key: &str) -> Result<String, JoseError> {
    let mut header = JwsHeader::new();
    header.set_algorithm("HS256");
    header.set_token_type("JWT");

    let mut payload = JwtPayload::new();
    payload.set_claim("id", Some(Value::String(guid.to_string())))?;

    let signer = HS256.signer_from_bytes(key)?;
    let jwt = encode_with_signer(&payload, &header, &signer)?;

    Ok(jwt)
}

fn gen_guid() -> String {
    GUID::rand().to_string().to_lowercase()
}

#[test]
fn test_jwt_sign_with_guid() -> Result<(), JoseError> {
    let jwt = jwt_sign_with_guid("06294495-2134-6603-e716-97ef9c0089a2", &JWT_KEY)?;
    assert_eq!(jwt,"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjA2Mjk0NDk1LTIxMzQtNjYwMy1lNzE2LTk3ZWY5YzAwODlhMiJ9.mb5eymv3yZtyGutvt9qeRkLVlHzA2pRrIJ-3m4QWLH4");
    let jwt = jwt_sign_with_guid("69b20cdd-6d77-0b32-1fd9-86fea6742863", &JWT_KEY)?;
    assert_eq!(jwt,"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjY5YjIwY2RkLTZkNzctMGIzMi0xZmQ5LTg2ZmVhNjc0Mjg2MyJ9.aUx3qNcrj7vXt0BT-ZkGVgOxnUBvru0rRVC22jEPRdk");
    Ok(())
}
