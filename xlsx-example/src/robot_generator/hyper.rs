use std::collections::HashMap;
use std::error::Error;

use hyper::client::HttpConnector;
use hyper::{Body, Client, Method, Request, Version};
use hyper_tls::HttpsConnector;

pub(crate) async fn request(
    url: &str,
    method: &Method,
    body: &Body,
    headers: &HashMap<String, String>,
) -> Result<(), Box<dyn Error>> {
    let mut req_builder = Request::builder();

    for (k, v) in headers {
        req_builder = req_builder.header(k, v);
    }

    req_builder = req_builder.method(method).uri(url).version(Version::HTTP_11);

    let client: Client<HttpConnector, Body>;

    if url.starts_with("https") {
        let client = Client::builder().build::<_, Body>(HttpsConnector::<HttpConnector>::new());

        /* let req = req_builder
        .header("user-agent", UA.to_string())
        .method(Method::GET)
        .uri(url)
        .version(Version::HTTP_11)
        .body(Body::empty())?; */
    }
    Ok(())
}
