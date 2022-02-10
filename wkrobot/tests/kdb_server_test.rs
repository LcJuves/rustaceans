include!("../src/lib.rs");

#[macro_use]
mod kbd_mod;

use crate::kbd_mod::*;

use core::time::Duration;

use std::collections::HashMap;
use std::error::Error;
use std::io::{stdout, ErrorKind, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;

use hyper::{Body, Client, Method, Request, Response, StatusCode, Version};
use lazy_static::lazy_static;
use tokio::runtime::Runtime;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    VK_0, VK_1, VK_2, VK_3, VK_4, VK_5, VK_6, VK_7, VK_8, VK_9, VK_A, VK_C, VK_E, VK_G, VK_H, VK_I,
    VK_J, VK_L, VK_N, VK_RETURN, VK_S, VK_SPACE, VK_U, VK_V,
};

lazy_static! {
    static ref TOKIO_RT: Runtime = Runtime::new().unwrap();
}

async fn request(
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

    let client = Client::new();
    let req = req_builder.body(body)?;

    Ok(client.request(req).await?)
}

async fn get(
    url: &str,
    headers: &HashMap<String, String>,
) -> Result<Response<Body>, Box<dyn Error>> {
    Ok(request(url, &Method::GET, Body::empty(), headers).await?)
}

async fn get_without_headers(url: &str) -> Result<Response<Body>, Box<dyn Error>> {
    Ok(get(url, &HashMap::new()).await?)
}

async fn test_kbd_req(frequency: u16, key_code: u16) -> Result<(), Box<dyn Error>> {
    let url = format!("http://127.0.0.1:3000/?frequency={}&keyCode={}", frequency, key_code);
    let resp = get_without_headers(&url).await?;
    assert_eq!(resp.status(), StatusCode::OK);
    assert_eq!(format!("{:?}", (*resp.body())), format!("{:?}", Body::empty()));
    Ok(())
}

#[test]
fn test_server_kbd() -> Result<(), Box<dyn Error>> {
    let cargo_manifest_dir_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let kbd_archive_path = _compile_kbd_exe()?;

    thread::spawn(move || {
        let cmd_status = Command::new("cargo")
            .arg("run")
            .current_dir(cargo_manifest_dir_path)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .unwrap();
        assert!(cmd_status.success());
    });

    let mut kbdbin_child = Command::new(&kbd_archive_path).stdout(Stdio::piped()).spawn()?;

    thread::spawn(|| {
        thread::sleep(Duration::from_millis(300));
        TOKIO_RT.block_on(test_kbd_req(2, VK_A)).unwrap();
        TOKIO_RT.block_on(test_kbd_req(1, VK_RETURN)).unwrap();
    });

    let output_string = _read_child_output_string!(kbdbin_child);
    kbdbin_child.wait()?;
    assert_eq!(output_string, "aa");

    let mut kbdbin_child = Command::new(&kbd_archive_path).stdout(Stdio::piped()).spawn()?;

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(300));
        TOKIO_RT.block_on(test_kbd_req(1, VK_L)).unwrap();
        TOKIO_RT.block_on(test_kbd_req(1, VK_I)).unwrap();
        TOKIO_RT.block_on(test_kbd_req(1, VK_A)).unwrap();
        TOKIO_RT.block_on(test_kbd_req(1, VK_N)).unwrap();
        TOKIO_RT.block_on(test_kbd_req(1, VK_G)).unwrap();
        TOKIO_RT.block_on(test_kbd_req(1, VK_C)).unwrap();
        TOKIO_RT.block_on(test_kbd_req(1, VK_H)).unwrap();
        TOKIO_RT.block_on(test_kbd_req(1, VK_E)).unwrap();
        TOKIO_RT.block_on(test_kbd_req(1, VK_N)).unwrap();
        TOKIO_RT.block_on(test_kbd_req(1, VK_G)).unwrap();
        TOKIO_RT.block_on(test_kbd_req(1, VK_SPACE)).unwrap();
        TOKIO_RT.block_on(test_kbd_req(1, VK_J)).unwrap();
        TOKIO_RT.block_on(test_kbd_req(1, VK_U)).unwrap();
        TOKIO_RT.block_on(test_kbd_req(1, VK_V)).unwrap();
        TOKIO_RT.block_on(test_kbd_req(1, VK_E)).unwrap();
        TOKIO_RT.block_on(test_kbd_req(1, VK_S)).unwrap();
        TOKIO_RT.block_on(test_kbd_req(1, VK_RETURN)).unwrap();
    });

    let output_string = _read_child_output_string!(kbdbin_child);
    kbdbin_child.wait()?;
    assert_eq!(output_string, "liangcheng juves");

    let mut kbdbin_child = Command::new(&kbd_archive_path).stdout(Stdio::piped()).spawn()?;

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(300));
        TOKIO_RT.block_on(test_kbd_req(1, VK_0)).unwrap();
        TOKIO_RT.block_on(test_kbd_req(1, VK_1)).unwrap();
        TOKIO_RT.block_on(test_kbd_req(1, VK_2)).unwrap();
        TOKIO_RT.block_on(test_kbd_req(1, VK_3)).unwrap();
        TOKIO_RT.block_on(test_kbd_req(1, VK_4)).unwrap();
        TOKIO_RT.block_on(test_kbd_req(1, VK_5)).unwrap();
        TOKIO_RT.block_on(test_kbd_req(1, VK_6)).unwrap();
        TOKIO_RT.block_on(test_kbd_req(1, VK_7)).unwrap();
        TOKIO_RT.block_on(test_kbd_req(1, VK_8)).unwrap();
        TOKIO_RT.block_on(test_kbd_req(1, VK_9)).unwrap();
        TOKIO_RT.block_on(test_kbd_req(1, VK_RETURN)).unwrap();
    });

    let output_string = _read_child_output_string!(kbdbin_child);
    kbdbin_child.wait()?;
    assert_eq!(output_string, "0123456789");

    std::fs::remove_file(&kbd_archive_path)?;

    Ok(())
}
