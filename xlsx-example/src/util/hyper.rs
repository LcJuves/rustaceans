use crate::seeval;

use std::collections::HashMap;
use std::error::Error;

use hyper::body::Buf;
use hyper::{Body, Client, Method, Request, Response, Version};
use hyper_rustls::HttpsConnectorBuilder;
use serde_json::Value;

#[allow(unused_macros)]
#[macro_export(local_inner_macros)]
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

pub(crate) async fn request(
    url: &str,
    method: &Method,
    body: Body,
    headers: &HashMap<String, String>,
) -> Result<Response<Body>, Box<dyn Error>> {
    let mut req_builder = Request::builder();
    for (k, v) in headers {
        req_builder = req_builder.header(k, v);
    }
    req_builder = req_builder.method(method).uri(url).version(Version::HTTP_11);

    // https://github.com/seanmonstar/reqwest/issues/1310
    // use rustls::ciphersuite::TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384;
    let client_config = hyper_rustls::rustls::ClientConfig::builder()
        .with_safe_default_cipher_suites()
        .with_safe_default_kx_groups()
        .with_safe_default_protocol_versions()
        .unwrap()
        .with_no_certificate_verifier()
        .with_no_client_auth();
    let client = Client::builder().build::<_, Body>(
        HttpsConnectorBuilder::new()
            .with_tls_config(client_config)
            .https_or_http()
            .enable_http1()
            .build(),
    );

    let req = req_builder.body(body)?;

    Ok(client.request(req).await?)
}

pub(crate) async fn get(
    url: &str,
    headers: &HashMap<String, String>,
) -> Result<Response<Body>, Box<dyn Error>> {
    seeval!(url);
    seeval!(headers);
    Ok(request(url, &Method::GET, Body::empty(), headers).await?)
}

pub(crate) async fn get_without_headers(url: &str) -> Result<Response<Body>, Box<dyn Error>> {
    Ok(get(url, &HashMap::new()).await?)
}

pub(crate) async fn post(
    url: &str,
    body: Body,
    headers: &HashMap<String, String>,
) -> Result<Response<Body>, Box<dyn Error>> {
    seeval!(url);
    seeval!(body);
    seeval!(headers);
    Ok(request(url, &Method::POST, body, headers).await?)
}

pub(crate) async fn resp_body_bytes_from(resp: Response<Body>) -> Result<Vec<u8>, Box<dyn Error>> {
    let resp_body = hyper::body::aggregate(resp).await?;
    let mut resp_json_bytes = Vec::new();
    std::io::copy(&mut resp_body.reader(), &mut resp_json_bytes)?;
    Ok(resp_json_bytes)
}

pub(crate) async fn resp_json_string_from(resp: Response<Body>) -> Result<String, Box<dyn Error>> {
    let resp_body_bytes = resp_body_bytes_from(resp).await?;
    let resp_json_string = String::from_utf8_lossy(&resp_body_bytes);
    let resp_json_string = if resp_json_string.contains("':") || resp_json_string.contains("' :") {
        resp_json_string.replace("'", r#"""#)
    } else {
        resp_json_string.to_string()
    };
    Ok(resp_json_string)
}

pub(crate) async fn resp_json_from(resp: Response<Body>) -> Result<Value, Box<dyn Error>> {
    Ok(serde_json::from_str(&(resp_json_string_from(resp).await?))?)
}
