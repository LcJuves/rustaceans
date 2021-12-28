use std::collections::HashMap;
use std::error::Error;

use hyper::{Body, Client, Method, Request, Response, Version};
use hyper_tls::HttpsConnector;

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

pub(crate) async fn post(
    url: &str,
    body: Body,
    headers: &HashMap<String, String>,
) -> Result<Response<Body>, Box<dyn Error>> {
    Ok(request(url, &Method::POST, body, headers).await?)
}
