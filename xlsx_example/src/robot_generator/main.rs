use crate::robot_generator::cli_parser::*;
use crate::robot_generator::one_case::OneCase;
use crate::util::calamine_util::*;

use std::env::{args_os, current_dir};
use std::ffi::OsString;
use std::path::Path;

use calamine::{open_workbook_auto, Error, Sheets};

pub(crate) fn robot_generator_main() -> Result<(), Error> {
    let args_vec = args_os().collect::<Vec<OsString>>();
    if args_vec.len() < 2 {
        eprintln!("At least one command line parameter needs to be passed!");
        std::process::exit(-1);
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
        let ref cli_matches = CLI_MATCHES;
        let xlsx_url = cli_matches.value_of("xlsx-url").unwrap_or("");
        let xlsx_path = cli_matches.value_of("xlsx-path").unwrap_or("");

        workbook = if !xlsx_url.trim().is_empty() {
            open_workbook_by_url(&xlsx_url)?
        } else {
            open_workbook_auto(Path::new(&xlsx_path))?
        }
    }

    if let Some(default_sheet) = default_sheet_of_wb(&mut workbook) {
        if let Ok(cases) = sheet_reflect::<OneCase>(&default_sheet) {
            for mut one_case in cases {
                one_case.save_robot_to(&current_dir().unwrap())?;
            }
        }
    }

    Ok(())
}
