use crate::robot_generator::cli_parser::*;
use crate::robot_generator::one_case::{OneCase, ROBOT_TEMPLATE};
use crate::robot_generator::tpauth::*;
use crate::robot_generator::tputil::*;
use crate::util::calamine::*;

use std::env::{args_os, current_dir};
use std::error::Error;
use std::ffi::OsString;
use std::fs::OpenOptions;
use std::path::Path;

use calamine::{open_workbook_auto, Sheets};

pub(crate) fn robot_generator_main() -> Result<(), Box<dyn Error>> {
    let args_vec = args_os().collect::<Vec<OsString>>();
    if args_vec.len() < 2 {
        println!(
            "\u{1b}[91m{}\u{1b}[0m",
            "At least one command line parameter needs to be passed!"
        );
        println!(
            "For more information try \u{1b}[92m{}\u{1b}[0m or \u{1b}[92m{}\u{1b}[0m",
            "--help", "-h"
        );
        std::process::exit(-1);
    }

    if args_os_has_flag("--login") {
        let (ep_jwt_token_current, sessionid, username, email, staff_code, token) =
            (TOKIO_RT.as_ref()?).block_on(sign_in_tp_by_scan_moa_arcode())?;
        if args_vec.len() == 2 {
            let auth_conf = AUTH_CONF.as_ref()?;
            if !ep_jwt_token_current.is_empty()
                && !sessionid.is_empty()
                && !username.is_empty()
                && !email.is_empty()
                && !staff_code.is_empty()
                && !token.is_empty()
            {
                println!("{:#?}", auth_conf);
                println!("Login successful!");
            }
            return Ok(());
        }
    }

    let arg_1 = &args_vec[1];
    let arg_1_string = &arg_1.clone().into_string().unwrap();

    let mut workbook: Sheets;

    if !arg_1_string.starts_with("-") {
        workbook = if arg_1_string.starts_with("http") {
            let wb_url = &arg_1_string.clone();
            open_workbook_by_url(&wb_url)?
        } else {
            let wb_path = &arg_1.clone();
            open_workbook_auto(Path::new(wb_path))?
        };
    } else {
        if let Some(export_temp_path_os_string) = option_value_of("--export-def-temp") {
            let export_temp_path = Path::new(&export_temp_path_os_string);
            if !export_temp_path.exists() {
                if let None = export_temp_path.extension() {
                    if let Err(e) = std::fs::create_dir_all(&export_temp_path) {
                        eprintln!("{}", e);
                    }
                }
            }
            let export_temp_path = if export_temp_path.is_dir() {
                export_temp_path.join("case.temp")
            } else {
                export_temp_path.to_path_buf()
            };
            let mut temp_file = OpenOptions::new()
                .create(true)
                .read(true)
                .write(true)
                .truncate(true)
                .open(&export_temp_path)?;
            let robot_template = &*ROBOT_TEMPLATE;
            std::io::copy(&mut robot_template.as_bytes(), &mut temp_file)?;
            println!("Export template to `{}` successfully", &export_temp_path.to_string_lossy());
            return Ok(());
        }

        let ref cli_matches = CLI_MATCHES;
        let xlsx_url = cli_matches.value_of("xlsx-url").unwrap_or("");
        let xlsx_path = cli_matches.value_of("xlsx-path").unwrap_or("");

        workbook = if !xlsx_url.trim().is_empty() {
            if !xlsx_url.trim_start().starts_with("http") {
                println!("Error option with --xlsx-url, is not a valid URL");
                std::process::exit(-1);
            }
            open_workbook_by_url(&xlsx_url)?
        } else {
            open_workbook_auto(Path::new(&xlsx_path))?
        }
    }

    if let Some(default_sheet) = default_sheet_of_wb(&mut workbook) {
        let save_robot_dir = if let Some(option_out_dir_os_string) = option_value_of("--out-dir") {
            Path::new(&option_out_dir_os_string).to_path_buf()
        } else {
            current_dir()?
        };
        if let Ok(cases) = sheet_reflect::<OneCase>(&default_sheet) {
            for mut one_case in cases {
                one_case.save_robot_to(&save_robot_dir)?;
            }
        }
    }

    Ok(())
}
