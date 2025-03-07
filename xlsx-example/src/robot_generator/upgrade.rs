//! Created at 2022/1/1 14:14
//! @author Liangcheng Juves

use crate::robot_generator::cli_parser::option_value_of;
use crate::seeval;
use crate::util::hyper::*;

use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

use awaits::future_block_on;
use hyper::StatusCode;
use lazy_static::lazy_static;
use ruimpl::rmeol;
use serde::{Deserialize, Serialize};
use ssri::{Algorithm, IntegrityOpts};

lazy_static! {
    pub(crate) static ref UPGRADE_HOST: String = {
        if let Some(upgrade_host) = option_value_of("--upgrade-host") {
            upgrade_host.to_str().unwrap().to_owned()
        } else {
            #[cfg(debug_assertions)]
            let default_upgrade_host = "rexe-upgrade.io:9934";
            #[cfg(not(debug_assertions))]
            let default_upgrade_host = "172.22.228.187:9934";

            default_upgrade_host.to_owned()
        }
    };
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct LatestInfo {
    pub(crate) name: String,
    pub(crate) version: String,
}

pub(crate) async fn get_upgrade_latest_info() -> Result<LatestInfo, Box<dyn Error>> {
    let resp = get_without_headers(&format!("http://{}/latest.json", &*UPGRADE_HOST)).await?;
    let latest_info: LatestInfo = serde_json::from_str(&(resp_json_string_from(resp).await?))?;
    Ok(latest_info)
}

pub(crate) fn compute_sha512sum(bytes: &[u8]) -> String {
    let integrity = IntegrityOpts::new().algorithm(Algorithm::Sha512).chain(bytes).result();
    let (_, hex) = integrity.to_hex();
    hex
}

pub(crate) async fn dl_and_check_latest_exec_file(
    latest_info: &LatestInfo,
) -> Result<(), Box<dyn Error>> {
    let platf_name = if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(any(target_os = "linux", target_os = "l4re")) {
        "linux"
    } else if cfg!(windows) {
        "windows"
    } else {
        panic!("Unsupported platform")
    };
    let exec_file_url = format!(
        "http://{}/{}/{}-{}",
        &*UPGRADE_HOST, platf_name, latest_info.name, latest_info.version
    );
    #[cfg(windows)]
    let exec_file_url = exec_file_url + ".exe";
    let exec_file_resp = get_without_headers(&exec_file_url).await?;
    if exec_file_resp.status() != StatusCode::OK {
        println!("GET {} \n{:#?}", exec_file_url, exec_file_resp);
        panic!();
    }
    let mut exec_file_content = resp_body_bytes_from(exec_file_resp).await?;

    let exec_file_sha512sum_url = format!("{}{}", exec_file_url, ".sha512");
    let exec_file_sha512sum_resp = get_without_headers(&exec_file_sha512sum_url).await?;
    if exec_file_sha512sum_resp.status() != StatusCode::OK {
        println!("GET {} \n{:#?}", exec_file_sha512sum_url, exec_file_sha512sum_resp);
        panic!();
    }
    let exec_file_sha512sum_content = resp_body_bytes_from(exec_file_sha512sum_resp).await?;
    let exec_file_sha512sum_content = rmeol(std::str::from_utf8(&exec_file_sha512sum_content)?);
    let exec_file_sha512sum = exec_file_sha512sum_content
        [0..(exec_file_sha512sum_content.rfind(" ").unwrap_or(exec_file_sha512sum_content.len()))]
        .trim();

    if compute_sha512sum(&exec_file_content) == exec_file_sha512sum {
        let mut curr_exe_file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .read(true)
            .write(true)
            .open(get_curr_exe_path()?)?;
        curr_exe_file.write_all(&mut exec_file_content)?;
        curr_exe_file.flush()?;
    } else {
        panic!();
    }

    Ok(())
}

pub(crate) fn self_upgrade() -> Result<(), Box<dyn Error>> {
    async fn upgrade() -> Result<(), Box<dyn Error>> {
        let latest_info = get_upgrade_latest_info().await?;
        seeval!(latest_info);
        let curr_exe_version = get_curr_exe_version()?;
        let latest_info_version = &latest_info.version;
        if u32::from_str(&latest_info_version.replace(".", ""))?
            < u32::from_str(&curr_exe_version.replace(".", ""))?
        {
            println!("The latest version number returned by the server is: {}, which is less than the current version number: {}; no need to update", latest_info_version, curr_exe_version);
        } else {
            dl_and_check_latest_exec_file(&latest_info).await?;
            println!(
                "Successfully updated from version {} to {}!",
                curr_exe_version, latest_info_version
            );
        }
        Ok(())
    }

    future_block_on!(upgrade())?;

    Ok(())
}

pub(crate) fn get_curr_exe_path() -> Result<PathBuf, std::io::Error> {
    #[cfg(debug_assertions)]
    println!("TARGET >>> {}", env!("TARGET"));
    let curr_exe_path = if cfg!(test) {
        let mut cargo_manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        seeval!(cargo_manifest_dir);
        let full_exe_name = env!("CARGO_BIN_NAME");
        #[cfg(windows)]
        let full_exe_name = full_exe_name.to_owned() + ".exe";
        if cargo_manifest_dir.parent().unwrap().join("Cargo.toml").exists() {
            cargo_manifest_dir = cargo_manifest_dir.parent().unwrap().to_path_buf();
        }
        cargo_manifest_dir.join("target").join("debug").join(full_exe_name)
    } else {
        use std::env::current_exe;
        current_exe()?
    };
    Ok(curr_exe_path)
}

pub(crate) fn get_curr_exe_version() -> Result<String, Box<dyn Error>> {
    let curr_exe_path = get_curr_exe_path()?;
    seeval!(curr_exe_path);
    let output = Command::new(curr_exe_path.as_os_str()).arg("-V").output()?;
    seeval!(output);
    let version_info_line = rmeol(std::str::from_utf8(&output.stdout)?);
    seeval!(version_info_line);
    let version = version_info_line[version_info_line.rfind(" ").unwrap_or(0)..].trim();
    seeval!(version);
    Ok(version.to_owned())
}

#[allow(dead_code)]
#[cfg(any(debug_assertions, test))]
pub(crate) fn get_cargo_toml_version() -> Result<String, Box<dyn Error>> {
    use toml::Value;

    let cargo_toml_str = include_str!("../../Cargo.toml");
    let value = cargo_toml_str.parse::<Value>()?;

    let cargo_toml_version = value["package"]["version"].as_str().unwrap();

    Ok(cargo_toml_version.to_owned())
}

#[test]
fn test_get_curr_exe_version() -> Result<(), Box<dyn Error>> {
    let cmd_status = Command::new("cargo")
        .arg("build")
        .current_dir(PathBuf::from(env!("CARGO_MANIFEST_DIR")))
        .status()?;
    assert!(cmd_status.success());

    let cargo_toml_version = get_cargo_toml_version()?;
    let curr_exe_version = get_curr_exe_version()?;

    assert_eq!(curr_exe_version, cargo_toml_version);
    Ok(())
}

#[test]
fn test_compute_sha512sum() {
    assert_eq!(compute_sha512sum(include_bytes!("../../../rustfmt.toml")), "35a6fad01772997cd82793cd8c3c4991f52581de87a0a9253db6158ff4724aaea7751aca4a74cafb71fc2377581ebcc4f738829c97587ecae5aa3c985825db34");
    assert_eq!(
        compute_sha512sum(include_bytes!("../../tests/res/upgrade/macos/xlsx-example-1.0.2")),
        rmeol(include_str!("../../tests/res/upgrade/macos/xlsx-example-1.0.2.sha512"))
    );
    assert_eq!(compute_sha512sum(include_bytes!("../../tests/res/exam0.xlsx")), "f75b8b5d5278f3eb5a07134ef19c1130da74bfc759c0d25fea5a391d1acef9ace6ca251e067f2c09bed400964afc0b818ede1fac518e29a2aadeada716e8edee");
}
