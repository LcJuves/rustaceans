use std::env::{args_os, current_dir};
use std::ffi::OsString;

use std::path::Path;

mod reflection;
mod robot_generator;
mod util;

use crate::robot_generator::one_case::OneCase;
use crate::util::calamine_util::*;

use calamine::{open_workbook_auto, Error};

fn main() -> Result<(), Error> {
    let args_vec = args_os().collect::<Vec<OsString>>();

    let arg_1 = &args_vec[1];
    let arg_1_string = &arg_1.clone().into_string().unwrap();
    let mut workbook = if arg_1_string.starts_with("http") {
        open_workbook_by_url(&arg_1_string)?
    } else {
        open_workbook_auto(Path::new(&arg_1))?
    };

    if let Some(default_sheet) = default_sheet_of_wb(&mut workbook) {
        if let Ok(cases) = sheet_reflect::<OneCase>(&default_sheet) {
            for mut one_case in cases {
                one_case.save_robot_to(&current_dir().unwrap())?;
            }
        }
    }

    Ok(())
}
