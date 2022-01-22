include!("../lib.rs");

use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

// TODO
async fn hello_world(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    if let Some(url_query_string) = req.uri().query() {
        let first_query_split = url_query_string.split("&").collect::<Vec<&str>>();
        let mut url_query_map = HashMap::<String, String>::new();
        for first_query in first_query_split {
            let second_split = first_query.split("=").collect::<Vec<&str>>();
            let k = second_split[0];
            let v = second_split[1];
            url_query_map.insert(k.to_owned(), v.to_owned());
        }
    }
    Ok(Response::new(Body::empty()))
}

#[tokio::main]
async fn main() {
    // We'll bind to 127.0.0.1:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `hello_world` function.
    let make_svc = make_service_fn(|_conn| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(hello_world))
    });

    println!("Server listening on {}", addr);
    let server = Server::bind(&addr).serve(make_svc);

    // Run this server for... forever!
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
