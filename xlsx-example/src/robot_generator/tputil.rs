// use rhexstr::HexString;

// use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{stdin, stdout, BufRead, Write};

use guid_create::GUID;
use josekit::jws::{JwsHeader, HS256};
use josekit::jwt::{encode_with_signer, JwtPayload};
use josekit::{JoseError, Value};

use hyper::body::Buf;
use hyper::{Body, Client, Method, Request, Version};
use hyper_tls::HttpsConnector;
// use serde::Deserialize;

use lazy_static::lazy_static;

lazy_static! {
    static ref UA: &'static str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.159 Safari/537.36";
    // static ref RID_PWD: &'static str = "32663266326832723286329432963316333833543372337634223430346435003538357036043640365436943760378038663882396439684046411841844252";
    static ref JWT_KEY: &'static str = "32293231323532373241325132713273328533033339335733613403343934413469350335713605364136513735376937813863393139494017409141594161";
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

pub(crate) async fn verify_sms_code(
    auth_token: &str,
    app_token: &str,
    session_id: &str,
    phone_num: &str,
    user_name: &str,
    sms_code: &u32,
) -> Result<(), Box<dyn std::error::Error>> {
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
                println!("location >>> {}", location);
            }
        } else {
            eprintln!("msg >>> {}", resp_json["msg"]);
            std::process::exit(-1);
        }
    }

    /* use hyper::body::HttpBody as _;
    use tokio::io::{stdout, AsyncWriteExt as _};

    // And now...
    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?;
    } */

    std::process::exit(-1);
}

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

/* pub(crate) async fn get_handshake(
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
} */

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

pub(crate) fn gen_guid() -> String {
    GUID::rand().to_string().to_lowercase()
}

// fn decrypt_rc4(src: &str, pwd: &str) -> String {
//     let src_bytes = HexString.parse(src);
//     let src_len = src_bytes.len();

//     let mut key = vec![0u32; 256];
//     let mut sbox = vec![0u32; 256];
//     let mut output = vec!['\0'; src_len];

//     let plen = pwd.len();
//     let pwd_chars = pwd.chars().collect::<Vec<char>>();
//     // println!("pwd_chars >>> {:?}", pwd_chars);

//     for i in 0..256 {
//         key[i] = pwd_chars[i % plen] as u32;
//         sbox[i] = i as u32;
//     }

//     // println!("key >>> {:?}", key);

//     let mut j = 0;
//     for i in 0..256 {
//         j = (j + sbox[i] + key[i]) % 256;
//         let temp = sbox[i];
//         sbox[i] = sbox[j as usize];
//         sbox[j as usize] = temp;
//     }

//     #[allow(unused_assignments)]
//     let [mut a, mut b, mut c] = [0, 0, 0];
//     for i in 0..src_len {
//         a = (a + 1) % 256;
//         b = (b + sbox[a as usize]) % 256;
//         let mut temp = sbox[a as usize];
//         sbox[a as usize] = sbox[b as usize];
//         sbox[b as usize] = temp;
//         c = (sbox[a as usize] + sbox[b as usize]) % 256;
//         temp = (src_bytes[i] as u32) ^ sbox[c as usize];
//         output[i] = char::from_u32(temp).unwrap();
//     }

//     output.iter().collect::<String>()
// }

// #[test]
// fn test_decrypt_rc4() {
//     let ret = decrypt_rc4("293A73881ADD7CD375AC7C1E2F5006F122560D9DB84975BE66B3EB321674B5CAD639C83FB8ED42E190F5EB084227E2B59086D35F8271845262A721D052BBC73EE014C715DA669EFC61F2FE731D919E36B7303FE5C2622A0DAE88C259E1C00D6DC294A8F53ADDCBA54B330C0425544EA38215245B1605F682D05252AACF416DC469937CDFCFFDB9A35A86DED314ABB7E3A335975509E47F07C8DBA2F38521364DEEB3149774EA43054978CBD506E82CC503A10578B96EE0160369AAAAFD30B0614D3933E3DDD85B810B2FB46976D9A0B1755A7E7ABF7F4A4457F481DE6AD1E9D8B2D1B98C3DFA9A216B1C5903668428F123358F5256E16B87D4F0C7672270E745","1638332799356");
//     assert_eq!(ret,"1724A2FA5698E5BECF644F6AFAC3138C0304EA1EAC1B74BE3FAFF2F291C08B2EE9F58CDF105C0B75AB629136BBECF9CABF6250910D552885D95DC86686DC7C92BD6C35439A3487EEC599FED9B278E4498EAC4F361D11AF98F39EE0A432851B71A1032C48462EC526E04230E1AFB3A06676F638F50748095B87814F5F617096B3");
//     assert_eq!(decrypt_rc4(&ret, &RID_PWD),"32293231323532373241325132713273328533033339335733613403343934413469350335713605364136513735376937813863393139494017409141594161");
//     let ret = decrypt_rc4("EFEE2233A8C9C4109E19963899620E40AC6B8A8C76189C5132AA6AFFA300A585E5CB476D609F720F4D09275FB0FAA2CAAA744A4DC648075C85317ED8C87340A059811B0C0A77D6CBE7C01B75122F49FF72D5CF7ABA4F985DF5517CEF521F49D13AE9219B60F989322A17A78C83DBF71D369C1EA2F74B47FE5B28067A0E7993EA2585754EB54DE304AF65058D83FAA1A2A965C8F4EEB5EA6E888003BC418AC37EF8763ED6BD2A9AC77E625F7BED8C596514AA08591595ADA6A8A103C3C9D850FD0C563A0C3E4B887FB7D6AAA9E1C50363003F61708703BA86C791E2E7C27D9ACE50100230ED0FC99037CD60AD00CE85B52253696B78CB6283BF5141A8E8E0601A","1638339767454");
//     assert_eq!(ret,"1724A2FA5698E5BECF644F6AFAC3138C0304EA1EAC1B74BE3FAFF2F291C08B2EE9F58CDF105C0B75AB629136BBECF9CABF6250910D552885D95DC86686DC7C92BD6C35439A3487EEC599FED9B278E4498EAC4F361D11AF98F39EE0A432851B71A1032C48462EC526E04230E1AFB3A06676F638F50748095B87814F5F617096B3");
//     assert_eq!(decrypt_rc4(&ret, &RID_PWD),"32293231323532373241325132713273328533033339335733613403343934413469350335713605364136513735376937813863393139494017409141594161");
// }

#[test]
fn test_jwt_sign_with_guid() -> Result<(), JoseError> {
    let jwt = jwt_sign_with_guid("06294495-2134-6603-e716-97ef9c0089a2", &JWT_KEY)?;
    assert_eq!(jwt,"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjA2Mjk0NDk1LTIxMzQtNjYwMy1lNzE2LTk3ZWY5YzAwODlhMiJ9.mb5eymv3yZtyGutvt9qeRkLVlHzA2pRrIJ-3m4QWLH4");
    let jwt = jwt_sign_with_guid("69b20cdd-6d77-0b32-1fd9-86fea6742863", &JWT_KEY)?;
    assert_eq!(jwt,"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjY5YjIwY2RkLTZkNzctMGIzMi0xZmQ5LTg2ZmVhNjc0Mjg2MyJ9.aUx3qNcrj7vXt0BT-ZkGVgOxnUBvru0rRVC22jEPRdk");
    Ok(())
}
