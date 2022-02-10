include!("../lib.rs");

use core::time::Duration;

use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::str::FromStr;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};

async fn hello_world(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let mut resp = Response::new(Body::empty());
    if req.uri().path() != "/" {
        *resp.status_mut() = StatusCode::from_u16(403).unwrap();
        return Ok(resp);
    }
    if let Some(url_query_string) = req.uri().query() {
        let url_query_split =
            url_query_string.split("&").filter(|s| !s.is_empty()).collect::<Vec<&str>>();
        let mut search_params = HashMap::<String, String>::new();
        for url_query in url_query_split {
            let url_query_split_index = url_query.find("=").unwrap();
            let k = &url_query[..url_query_split_index];
            let v = &url_query[(url_query_split_index + 1)..];
            search_params.insert(decode_uri(k), decode_uri(v));
        }

        let mut key_code = 0;
        if let Some(strval) = search_params.get("keyCode") {
            if let Ok(val) = u16::from_str(strval) {
                key_code = val;
            }
        }

        let mut frequency = 0;
        if let Some(strval) = search_params.get("frequency") {
            if let Ok(val) = u16::from_str(strval) {
                frequency = val;
            }
        }

        let mut focus_window_name = "";
        if let Some(strval) = search_params.get("focusWindowName") {
            focus_window_name = strval;
        }

        let mut wait_focus_time_millis = 1500;
        if let Some(strval) = search_params.get("waitFocusTimeMillis") {
            if let Ok(val) = u64::from_str(strval) {
                wait_focus_time_millis = val;
            }
        }

        if frequency <= 0 {
            *resp.body_mut() = Body::from("`frequency` should > 0");
            *resp.status_mut() = StatusCode::from_u16(403).unwrap();
            return Ok(resp);
        }

        let mut final_ret: windows::core::Result<()> = Ok(());

        if key_code > 0 {
            if !focus_window_name.is_empty() {
                println!("focusWindowName >>> {}", focus_window_name);
                final_ret = focus_window(focus_window_name);
                if wait_focus_time_millis > 0 {
                    println!("waitFocusTimeMillis >>> {}", wait_focus_time_millis);
                    std::thread::sleep(Duration::from_millis(wait_focus_time_millis));
                }
            }
            println!("keyCode >>> {key_code}; frequency >>> {frequency}");
            for _ in 0..frequency {
                final_ret = key_press(key_code);
            }
        }

        if let Err(e) = final_ret {
            *resp.body_mut() = Body::from(format!("InternalServerError: {}", e));
            *resp.status_mut() = StatusCode::from_u16(500).unwrap();
            return Ok(resp);
        }
    }
    Ok(resp)
}

fn decode_uri(uri: &str) -> String {
    let mut pending = 0u8;
    let mut digit = 0;
    let mut ret_bytes = Vec::<u8>::new();

    for uri_char in uri.chars() {
        if pending == 0 && uri_char != '%' {
            ret_bytes.push(uri_char as u8);
        } else {
            if pending == 0 {
                pending = 1;
            } else if pending == 1 {
                pending = 2;
                digit = uri_char.to_digit(16).unwrap();
                digit <<= 4;
            } else if pending == 2 {
                pending = 0;
                digit |= uri_char.to_digit(16).unwrap();
                ret_bytes.push(digit as u8);
                digit = 0;
            }
        }
    }

    String::from_utf8_lossy(&ret_bytes).to_string()
}

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    #[cfg(not(debug_assertions))]
    let addr = SocketAddr::from(([127, 0, 0, 1], 9934));

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

#[test]
fn test_decode_uri() {
    assert_eq!(
        decode_uri("https://mozilla.org/?x=%D1%88%D0%B5%D0%BB%D0%BB%D1%8B"),
        "https://mozilla.org/?x=шеллы"
    );
    assert_eq!(
        decode_uri(
            "https://developer.mozilla.org/ru/docs/JavaScript_%D1%88%D0%B5%D0%BB%D0%BB%D1%8B"
        ),
        "https://developer.mozilla.org/ru/docs/JavaScript_шеллы"
    );
}
