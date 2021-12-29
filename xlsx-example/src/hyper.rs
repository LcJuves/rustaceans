use std::collections::HashMap;
use std::error::Error;

use hyper::body::Buf;
use hyper::{Body, Client, Method, Request, Response, Version};
use hyper_tls::HttpsConnector;

use serde_json::Value;

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

    let client = Client::builder().build::<_, Body>(HttpsConnector::new());
    let req = req_builder.body(body)?;

    Ok(client.request(req).await?)
}

pub(crate) async fn get(
    url: &str,
    headers: &HashMap<String, String>,
) -> Result<Response<Body>, Box<dyn Error>> {
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
    Ok(request(url, &Method::POST, body, headers).await?)
}

pub(crate) async fn resp_json_from(resp: Response<Body>) -> Result<Value, Box<dyn Error>> {
    let resp_body = hyper::body::aggregate(resp).await?;
    let mut resp_json_bytes = Vec::new();
    std::io::copy(&mut resp_body.reader(), &mut resp_json_bytes)?;
    let resp_json_string = String::from_utf8_lossy(&resp_json_bytes);
    let resp_json_string = resp_json_string.replace("'", r#"""#);
    Ok(serde_json::from_str(&resp_json_string)?)
}
