use crate::common_util::remove_eol;
use crate::hyper::*;

use std::collections::BTreeMap;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{stdin, stdout, BufRead, Write};
use std::path::PathBuf;

use guid_create::GUID;
use hmac::{Hmac, NewMac};
use jwt::SignWithKey;
use sha2::Sha256;

use hyper::{Body, Client, Method, Request, Response, Version};

use qrcode::render::unicode;
use qrcode::QrCode;

use lazy_static::lazy_static;

#[allow(unused_macros)]
#[macro_export]
macro_rules! print_resp_body {
    ($resp:ident) => {
        use hyper::body::HttpBody as _;
        use tokio::io::{stdout, AsyncWriteExt as _};
        let mut $resp = $resp;
        while let Some(chunk) = $resp.body_mut().data().await {
            stdout().write_all(&chunk?).await?;
        }
        stdout().write_all(b"\n").await?;
    };
}

macro_rules! time_millis_string {
    () => {
        (|| {
            use std::time::{SystemTime, UNIX_EPOCH};
            SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis().to_string()
        })()
    };
}

#[macro_export]
macro_rules! seeval {
    ($val:expr) => {
        #[cfg(debug_assertions)]
        println!("{} >>> {:?}", stringify!($val), $val);
    };
}

#[allow(unused_macros)]
macro_rules! pass {
    () => {
        #[cfg(debug_assertions)]
        println!("\u{1b}[91m{}\u{1b}[0m", ">>> PASS");
    };
}

lazy_static! {
    static ref UA: &'static str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36";
    static ref JWT_KEY: &'static str = "32293231323532373241325132713273328533033339335733613403343934413469350335713605364136513735376937813863393139494017409141594161";
    static ref BR_UUID: &'static str = "014a6560486429cada00afc53fe1017c";
    pub static ref USER_INFO_JSON_PATH: std::io::Result<PathBuf> = {
        #[allow(deprecated)]
        if let Some(home_dir) = std::env::home_dir() {
            return Ok(home_dir.join(".user_info.json"));
        }
        panic!("Init `USER_INFO_JSON_PATH` with error");
    };
}

pub(crate) async fn req_api_v1_login() -> Result<(String, String), Box<dyn Error>> {
    let mut headers = HashMap::new();
    headers.insert("user-agent".to_owned(), UA.to_string());

    let resp = get("http://199.200.0.8/api/v1/login/", &headers).await?;
    seeval!(resp.headers());

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
) -> Result<(String, String), Box<dyn Error>> {
    let mut headers = HashMap::new();
    headers.insert("user-agent".to_owned(), UA.to_string());
    headers.insert("cookie".to_owned(), format!("sessionid={}", sessionid));

    let resp = get(url, &headers).await?;
    seeval!(resp.headers());

    seeval!(resp.headers());

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
) -> Result<(String, String, String), Box<dyn Error>> {
    let mut headers = HashMap::new();
    headers.insert("user-agent".to_owned(), UA.to_string());
    headers.insert("cookie".to_owned(), format!("_sso_provider_session={}", sso_provider_session));

    let resp = get(url, &headers).await?;
    seeval!(resp.headers());

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
) -> Result<(String, String, String), Box<dyn Error>> {
    let mut url = String::from(
        "https://idtrust.sangfor.com:444/ac_portal/userauth.html?template=default&client_id=",
    );
    url.push_str(client_id);
    url.push_str("&response_type=");
    url.push_str(response_type);
    url.push_str("&auth=oauth");

    let mut headers = HashMap::new();
    headers.insert("user-agent".to_owned(), UA.to_string());

    let resp = get(&url, &headers).await?;
    seeval!(resp.headers());

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

async fn common_req_ac_portal_login(
    authsessid: &str,
    body: Body,
) -> Result<Response<Body>, Box<dyn Error>> {
    let mut headers = HashMap::new();
    headers.insert("user-agent".to_owned(), UA.to_string());
    headers.insert("Cookie".to_owned(), format!("AUTHSESSID={}", authsessid));
    headers.insert(
        "Content-Type".to_owned(),
        "application/x-www-form-urlencoded; charset=UTF-8".to_owned(),
    );
    headers.insert("Zxy".to_owned(), jwt_sign_with_guid(&gen_guid(), &JWT_KEY)?);

    Ok(post(&"https://idtrust.sangfor.com:444/ac_portal/login.php", Body::from(body), &headers)
        .await?)
}

pub(crate) async fn req_vsms_ac_portal_login(
    auth_token: &str,
    app_token: &str,
    authsessid: &str,
    phone_num: &str,
) -> Result<String, Box<dyn Error>> {
    let mut body = String::from("authToken=");
    body.push_str(auth_token);
    body.push_str("&appToken=");
    body.push_str(app_token);
    body.push_str("&auth=oauth&template=default&uuid=");
    body.push_str(&BR_UUID);
    body.push_str("&opr=getSmsCode&phone=");
    body.push_str(phone_num);

    let resp = common_req_ac_portal_login(authsessid, Body::from(body)).await?;
    seeval!(resp.headers());

    let resp_json = resp_json_from(resp).await?;
    // seeval!(resp_json);

    if let serde_json::Value::Bool(success) = resp_json["success"] {
        if let serde_json::Value::String(msg) = &resp_json["msg"] {
            if success {
                if let serde_json::Value::String(user_name) = &resp_json["userName"] {
                    return Ok(user_name.to_owned());
                }
            } else {
                eprintln!("{}", msg);
                std::process::exit(-1);
            }
        }
    }

    std::process::exit(-1);
}

pub(crate) async fn verify_sms_req_ac_portal_login(
    auth_token: &str,
    app_token: &str,
    authsessid: &str,
    phone_num: &str,
    user_name: &str,
    sms_code: &u32,
) -> Result<String, Box<dyn Error>> {
    let mut body = String::from("authToken=");
    body.push_str(auth_token);
    body.push_str("&appToken=");
    body.push_str(app_token);
    body.push_str("&auth=oauth&template=default&uuid=");
    body.push_str(&BR_UUID);
    body.push_str("&opr=firstSmsLogin&phone=");
    body.push_str(phone_num);
    body.push_str("&smsCode=");
    body.push_str(&sms_code.to_string());
    body.push_str("&userName=");
    body.push_str(user_name);

    let resp = common_req_ac_portal_login(authsessid, Body::from(body)).await?;
    seeval!(resp.headers());

    let resp_json = resp_json_from(resp).await?;
    // seeval!(resp_json);

    if let serde_json::Value::Bool(success) = resp_json["success"] {
        if let serde_json::Value::String(msg) = &resp_json["msg"] {
            if success {
                if let serde_json::Value::String(location) = &resp_json["location"] {
                    return Ok(location.to_owned());
                }
            } else {
                eprintln!("{}", msg);
                std::process::exit(-1);
            }
        }
    }

    std::process::exit(-1);
}

pub(crate) async fn req_vscan_moa_qrcode_ac_portal_login(
    auth_token: &str,
    app_token: &str,
    authsessid: &str,
) -> Result<(String, String, String, String), Box<dyn Error>> {
    let mut body = String::from("authToken=");
    body.push_str(auth_token);
    body.push_str("&appToken=");
    body.push_str(app_token);
    body.push_str("&auth=oauth&template=default&uuid=");
    body.push_str(&BR_UUID);
    body.push_str("&opr=addAppAuth&app_auth_way=6");

    let resp = common_req_ac_portal_login(authsessid, Body::from(body)).await?;
    seeval!(resp.headers());

    let resp_json = resp_json_from(resp).await?;
    // seeval!(resp_json);

    if let serde_json::Value::Bool(success) = resp_json["success"] {
        if success {
            if let serde_json::Value::String(goto) = &resp_json["goto"] {
                let did_prefix = "did=";
                let session_prefix = "session=";
                let goto = goto.to_owned();
                let did = (&goto[(goto.find(did_prefix).unwrap() + did_prefix.len())
                    ..(goto.find(&format!("&{}", session_prefix)).unwrap())])
                    .to_string();
                let session = (&goto
                    [(goto.find(session_prefix).unwrap() + session_prefix.len())..])
                    .to_string();
                if let serde_json::Value::String(nonce) = &resp_json["nonce"] {
                    return Ok((did, session, nonce.to_owned(), goto));
                }
            }
        } else {
            if let serde_json::Value::String(json) = resp_json {
                eprintln!("{}", json);
            }
        }
    }

    std::process::exit(-1);
}

pub(crate) async fn verify_scan_moa_qrcode_req_ac_portal_login(
    auth_token: &str,
    app_token: &str,
    authsessid: &str,
    moa_state: &str,
    user_name: &str,
    nonce: &str,
) -> Result<String, Box<dyn Error>> {
    let mut body = String::from("authToken=");
    body.push_str(auth_token);
    body.push_str("&appToken=");
    body.push_str(app_token);
    body.push_str("&auth=oauth&template=default&uuid=");
    body.push_str(&BR_UUID);
    body.push_str("&opr=firstMoaLogin&moaState=");
    body.push_str(moa_state);
    body.push_str("&userName=");
    body.push_str(user_name);
    body.push_str("&nonce=");
    body.push_str(nonce);

    let resp = common_req_ac_portal_login(authsessid, Body::from(body)).await?;
    seeval!(resp.headers());

    let resp_json = resp_json_from(resp).await?;
    // seeval!(resp_json);

    if let serde_json::Value::Bool(success) = resp_json["success"] {
        if let serde_json::Value::String(msg) = &resp_json["msg"] {
            if success {
                if let serde_json::Value::String(location) = &resp_json["location"] {
                    return Ok(location.to_owned());
                }
            } else {
                eprintln!("{}", msg);
                std::process::exit(-1);
            }
        }
    }

    std::process::exit(-1);
}

async fn send_get_req_to_idtrust(
    url: &str,
    authsessid: &str,
) -> Result<Response<Body>, Box<dyn Error>> {
    let mut headers = HashMap::new();
    headers.insert("user-agent".to_owned(), UA.to_string());
    headers.insert("Cookie".to_owned(), format!("AUTHSESSID={}", authsessid));
    headers.insert("Connection".to_owned(), "keep-alive".to_owned());
    headers.insert("Zxy".to_owned(), jwt_sign_with_guid(&gen_guid(), &JWT_KEY)?);

    Ok(get(url, &headers).await?)
}

pub(crate) async fn req_cgi_bin_tdc_get(
    did: &str,
    session: &str,
    authsessid: &str,
) -> Result<(String, u64), Box<dyn Error>> {
    let mut url = String::from("https://idtrust.sangfor.com:444/cgi-bin/tdc/get?did=");
    url.push_str(did);
    url.push_str("&session=");
    url.push_str(session);
    url.push_str("&_=");
    url.push_str(&time_millis_string!());

    let resp = send_get_req_to_idtrust(&url, authsessid).await?;
    seeval!(resp.headers());

    let resp_json = resp_json_from(resp).await?;
    // seeval!(resp_json);

    if let serde_json::Value::Number(result) = &resp_json["result"] {
        if let serde_json::Value::String(errmsg) = &resp_json["errmsg"] {
            if result.as_u64().unwrap() == 0 {
                if let serde_json::Value::Object(data) = &resp_json["data"] {
                    let tdc_info = data.get("tdc_info").unwrap().as_str().unwrap().to_owned();
                    let time_limit = data.get("time_limit").unwrap().as_u64().unwrap();
                    return Ok((tdc_info, time_limit));
                }
            } else {
                eprintln!("{}", errmsg);
                std::process::exit(-1);
            }
        }
    }

    std::process::exit(-1);
}

pub(crate) async fn req_cgi_bin_tdc_wait(
    did: &str,
    session: &str,
    authsessid: &str,
) -> Result<String, Box<dyn Error>> {
    let mut url = String::from("https://idtrust.sangfor.com:444/cgi-bin/tdc/wait?did=");
    url.push_str(did);
    url.push_str("&session=");
    url.push_str(session);
    url.push_str("&_=");
    url.push_str(&time_millis_string!());

    let resp = send_get_req_to_idtrust(&url, authsessid).await?;
    seeval!(resp.headers());

    let resp_json = resp_json_from(resp).await?;
    // seeval!(resp_json);

    if let serde_json::Value::Number(result) = &resp_json["result"] {
        if let serde_json::Value::String(errmsg) = &resp_json["errmsg"] {
            if result.as_u64().unwrap() == 0 {
                if let serde_json::Value::Object(data) = &resp_json["data"] {
                    let url = data.get("url").unwrap().as_str().unwrap().to_owned();
                    return Ok((&url[url.find("?").unwrap()..]).to_string());
                }
            } else {
                eprintln!("{}", errmsg);
                std::process::exit(-1);
            }
        }
    }

    std::process::exit(-1);
}

pub(crate) async fn req_ac_portal_auth_moa(
    authsessid: &str,
    search_params: &str,
) -> Result<(String, String), Box<dyn Error>> {
    let mut url = String::from("https://idtrust.sangfor.com:444/ac_portal/auth/moa.php");
    url.push_str(search_params);

    let resp = send_get_req_to_idtrust(&url, authsessid).await?;
    seeval!(resp.headers());

    let resp_json = resp_json_from(resp).await?;
    // seeval!(resp_json);

    if let serde_json::Value::Bool(success) = resp_json["success"] {
        if success {
            if let serde_json::Value::String(moa_state) = &resp_json["moaState"] {
                if let serde_json::Value::String(user_name) = &resp_json["userName"] {
                    return Ok((moa_state.to_owned(), user_name.to_owned()));
                }
            }
        } else {
            if let serde_json::Value::String(json) = resp_json {
                eprintln!("{}", json);
            }
        }
    }

    std::process::exit(-1);
}

pub(crate) async fn req_ss_login(
    url: &str,
    sso_provider_session: &str,
) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let req = Request::builder()
        .header("user-agent", UA.to_string())
        .header("cookie", format!("_sso_provider_session={}", sso_provider_session))
        .method(Method::GET)
        .uri(url)
        .version(Version::HTTP_11)
        .body(Body::empty())?;

    let resp = client.request(req).await?;
    seeval!(resp.headers());

    let redirect_url = String::from_utf8_lossy(resp.headers()["location"].as_bytes()).to_string();

    Ok(redirect_url)
}

pub(crate) async fn req_api_v1_login_callback(
    url: &str,
    sessionid: &str,
) -> Result<String, Box<dyn Error>> {
    let mut headers = HashMap::new();
    headers.insert("user-agent".to_owned(), UA.to_string());
    headers.insert("cookie".to_owned(), format!("sessionid={}", sessionid));

    let resp = get(url, &headers).await?;
    seeval!(resp.headers());

    let ep_jwt_token_current =
        String::from_utf8_lossy(resp.headers()["set-cookie"].as_bytes()).to_string();
    let ep_jwt_token_current = (&ep_jwt_token_current
        [(ep_jwt_token_current.find("=").unwrap() + 1)..(ep_jwt_token_current.find(";").unwrap())])
        .to_string();

    Ok(ep_jwt_token_current)
}

pub(crate) async fn req_api_v1_users_info(
    ep_jwt_token_current: &str,
    sessionid: &str,
) -> Result<(String, String, String, String, String), Box<dyn Error>> {
    let mut headers = HashMap::new();
    headers.insert("user-agent".to_owned(), UA.to_string());
    headers.insert(
        "cookie".to_owned(),
        format!("sessionid={}; ep_jwt_token_current={}", sessionid, ep_jwt_token_current),
    );

    let url = format!("http://199.200.0.8/api/v1/users/info/?_t={}", time_millis_string!());
    let resp = get(&url, &headers).await?;
    seeval!(resp.headers());

    let ep_jwt_token_current =
        String::from_utf8_lossy(resp.headers()["set-cookie"].as_bytes()).to_string();
    let ep_jwt_token_current = (&ep_jwt_token_current
        [(ep_jwt_token_current.find("=").unwrap() + 1)..(ep_jwt_token_current.find(";").unwrap())])
        .to_string();

    let resp_json = resp_json_from(resp).await?;
    // seeval!(resp_json);

    if let serde_json::Value::Number(id) = &resp_json["id"] {
        if id.as_u64().unwrap() > 0 {
            if let serde_json::Value::String(username) = &resp_json["username"] {
                if let serde_json::Value::String(email) = &resp_json["email"] {
                    if let serde_json::Value::String(staff_code) = &resp_json["staff_code"] {
                        if let serde_json::Value::String(token) = &resp_json["token"] {
                            return Ok((
                                ep_jwt_token_current.to_owned(),
                                username.to_owned(),
                                email.to_owned(),
                                staff_code.to_owned(),
                                token.to_owned(),
                            ));
                        }
                    }
                }
            }
        } else {
            if let serde_json::Value::String(json) = resp_json {
                eprintln!("{}", json);
            }
        }
    }

    std::process::exit(-1);
}

////////////////////////////////////////////////////////
////////////////////////////////////////////////////////
////////////////////////////////////////////////////////

pub(crate) async fn sign_in_tp_by_sms(
    phone_num: &str,
) -> Result<(String, String, String, String, String, String), Box<dyn Error>> {
    let (redirect_url, sessionid) = req_api_v1_login().await?;

    let (redirect_url, sso_provider_session) =
        req_ss_auth_att_oauth2_authorize(&redirect_url, &sessionid).await?;

    let (client_id, response_type, sso_provider_session) =
        req_ss_users_sign_in(&redirect_url, &sso_provider_session).await?;

    let (auth_token, app_token, authsessid) =
        req_ac_portal_userauth(&client_id, &response_type).await?;

    let user_name =
        req_vsms_ac_portal_login(&auth_token, &app_token, &authsessid, phone_num).await?;

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

    let (ep_jwt_token_current, username, email, staff_code, token) =
        req_api_v1_users_info(&ep_jwt_token_current, &sessionid).await?;
    seeval!((&ep_jwt_token_current, &username, &email, &staff_code, &token));

    save_user_info_json(&ep_jwt_token_current, &sessionid, &username, &email, &staff_code, &token)?;

    Ok((ep_jwt_token_current, sessionid, user_name, email, staff_code, token))
}

pub(crate) async fn sign_in_tp_by_scan_moa_arcode(
) -> Result<(String, String, String, String, String, String), Box<dyn Error>> {
    let (redirect_url, sessionid) = req_api_v1_login().await?;
    // seeval!((&redirect_url, &sessionid));

    let (redirect_url, sso_provider_session) =
        req_ss_auth_att_oauth2_authorize(&redirect_url, &sessionid).await?;
    // seeval!((&redirect_url, &sso_provider_session));

    let (client_id, response_type, sso_provider_session) =
        req_ss_users_sign_in(&redirect_url, &sso_provider_session).await?;
    // seeval!((&client_id, &response_type, &sso_provider_session));

    let (auth_token, app_token, authsessid) =
        req_ac_portal_userauth(&client_id, &response_type).await?;
    // seeval!((&auth_token, &app_token, &authsessid));

    let (did, session, nonce, ..) =
        req_vscan_moa_qrcode_ac_portal_login(&auth_token, &app_token, &authsessid).await?;
    // seeval!((&did, &session));

    let (tdc_info, _time_limit) = req_cgi_bin_tdc_get(&did, &session, &authsessid).await?;
    // seeval!((&tdc_info, &time_limit));

    let code = QrCode::new(tdc_info)?;
    let image = code
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    clearscreen::clear()?;
    println!("Please use MOA to scan the following QR code");
    println!();
    println!("{}", image);

    let search_params = req_cgi_bin_tdc_wait(&did, &session, &authsessid).await?;
    // seeval!(&search_params);

    let (moa_state, user_name) = req_ac_portal_auth_moa(&authsessid, &search_params).await?;
    // seeval!((&moa_state, &user_name));

    let redirect_url = verify_scan_moa_qrcode_req_ac_portal_login(
        &auth_token,
        &app_token,
        &authsessid,
        &moa_state,
        &user_name,
        &nonce,
    )
    .await?;

    let redirect_url = req_ss_login(&redirect_url, &sso_provider_session).await?;
    let ep_jwt_token_current = req_api_v1_login_callback(&redirect_url, &sessionid).await?;

    let (ep_jwt_token_current, username, email, staff_code, token) =
        req_api_v1_users_info(&ep_jwt_token_current, &sessionid).await?;
    seeval!((&ep_jwt_token_current, &username, &email, &staff_code, &token));

    save_user_info_json(&ep_jwt_token_current, &sessionid, &username, &email, &staff_code, &token)?;

    Ok((ep_jwt_token_current, sessionid, user_name, email, staff_code, token))
}

////////////////////////////////////////////////////////
////////////////////////////////////////////////////////
////////////////////////////////////////////////////////

pub fn read_stdin_sms_code() -> std::io::Result<u32> {
    let mut sms_code = String::new();

    stdout().write(b"Please enter the SMS verification code you received: ")?;
    stdout().flush()?;
    stdin().lock().read_line(&mut sms_code)?;

    let sms_code = remove_eol(&sms_code);

    use std::str::FromStr;
    Ok(u32::from_str(&sms_code).unwrap())
}

pub(crate) fn jwt_sign_with_guid(guid: &str, key: &str) -> Result<String, jwt::Error> {
    let key: Hmac<Sha256> = Hmac::new_from_slice(key.as_bytes())?;
    let mut claims = BTreeMap::new();
    claims.insert("alg", "HS256");
    claims.insert("typ", "JWT");
    claims.insert("id", guid);

    Ok(claims.sign_with_key(&key)?)
}

fn gen_guid() -> String {
    GUID::rand().to_string().to_lowercase()
}

pub fn user_info_json_exist() -> bool {
    if let Ok(user_info_json_path) = USER_INFO_JSON_PATH.as_ref() {
        if user_info_json_path.exists() {
            return true;
        }
    }
    false
}

pub fn save_user_info_json(
    ep_jwt_token_current: &str,
    sessionid: &str,
    user_name: &str,
    email: &str,
    staff_code: &str,
    token: &str,
) -> Result<(), Box<dyn Error>> {
    if !user_info_json_exist() {
        use serde_json::json;
        let user_info_json_path = USER_INFO_JSON_PATH.as_ref()?;
        let mut user_info_json = File::create(&user_info_json_path)?;
        let json = json!({
            "ep_jwt_token_current": ep_jwt_token_current,
            "sessionid": sessionid,
            "user_name": user_name,
            "email": email,
            "staff_code": staff_code,
            "token": token
        })
        .to_string();
        user_info_json.write_all(&json.as_bytes())?;
        user_info_json.flush()?;
    }
    Ok(())
}

#[test]
fn test_jwt_sign_with_guid() -> Result<(), Box<dyn Error>> {
    let jwt = jwt_sign_with_guid("06294495-2134-6603-e716-97ef9c0089a2", &JWT_KEY)?;
    assert_eq!(jwt,"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjA2Mjk0NDk1LTIxMzQtNjYwMy1lNzE2LTk3ZWY5YzAwODlhMiJ9.mb5eymv3yZtyGutvt9qeRkLVlHzA2pRrIJ-3m4QWLH4");
    let jwt = jwt_sign_with_guid("69b20cdd-6d77-0b32-1fd9-86fea6742863", &JWT_KEY)?;
    assert_eq!(jwt,"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjY5YjIwY2RkLTZkNzctMGIzMi0xZmQ5LTg2ZmVhNjc0Mjg2MyJ9.aUx3qNcrj7vXt0BT-ZkGVgOxnUBvru0rRVC22jEPRdk");
    Ok(())
}
