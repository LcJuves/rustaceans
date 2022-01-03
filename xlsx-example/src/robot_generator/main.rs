use crate::robot_generator::cli_parser::*;
use crate::robot_generator::one_case::{OneCase, ROBOT_TEMPLATE};
use crate::robot_generator::tpauth::*;
use crate::robot_generator::tputil::*;
use crate::robot_generator::upgrade::get_curr_exe_path;
use crate::robot_generator::upgrade::self_upgrade;
use crate::seeval;
use crate::util::calamine::*;
use crate::util::common::remove_eol;

use std::env::{args_os, current_dir};
use std::error::Error;
use std::ffi::OsString;
use std::fs::OpenOptions;
use std::path::Path;
use std::process::{Command, Stdio};

use calamine::{open_workbook_auto, Sheets};

fn exit_with_info(info: &str) {
    println!("\u{1b}[91m{}\u{1b}[0m", info);
    println!(
        "For more information try \u{1b}[92m{}\u{1b}[0m or \u{1b}[92m{}\u{1b}[0m",
        "--help", "-h"
    );
    std::process::exit(-1);
}

pub(crate) fn robot_generator_main() -> Result<(), Box<dyn Error>> {
    let args_vec = args_os().collect::<Vec<OsString>>();
    if args_vec.len() < 2 {
        exit_with_info("At least one command line parameter needs to be passed!");
    }

    if args_os_has_flag("--upgrade") {
        self_upgrade()?;
        return Ok(());
    }

    if args_os_has_flag("--amtpv") {
        add_me_to_path_var()?;
        return Ok(());
    }

    if args_os_has_flag("--set-alias") {
        if let Some(alias_name) = option_value_of("--set-alias") {
            set_alias(alias_name.to_str().unwrap())?;
            return Ok(());
        } else {
            exit_with_info("Option `--set-alias` needs to be followed by a value!");
        }
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

pub(crate) fn add_me_to_path_var() -> Result<(), Box<dyn Error>> {
    #[cfg(any(unix, target_os = "hermit"))]
    {
        eprintln!("Not currently supported!");
        return Ok(());
    }

    let curr_exe_path = get_curr_exe_path()?;
    seeval!(curr_exe_path);

    let rexe_home = curr_exe_path.as_path().parent().unwrap();
    seeval!(rexe_home);

    #[cfg(windows)]
    {
        let path = (|| -> Result<String, Box<dyn Error>> {
            let cmd_stdout = Command::new("cmd")
                .arg("/c")
                .arg("REG QUERY HKCU\\Environment /v PATH | findstr PATH")
                .output()?
                .stdout;
            let cmd_stdout = remove_eol(std::str::from_utf8(&cmd_stdout)?);
            let cmd_stdout = cmd_stdout[(cmd_stdout.rfind("_SZ ").unwrap_or(0) + 4)..].trim();
            let mut cmd_stdout = cmd_stdout.to_owned();
            assert!(!cmd_stdout.is_empty());
            if !cmd_stdout.ends_with(";") {
                cmd_stdout = cmd_stdout + ";";
            }

            Ok(cmd_stdout)
        })()?;

        seeval!(path);
    }

    #[cfg(windows)]
    {
        let cmd_status = Command::new("setx")
            .arg("REXE_HOME")
            .arg(rexe_home.to_str().unwrap())
            .stdout(Stdio::null())
            .status()?;
        assert!(cmd_status.success());
        if !path.contains("REXE_HOME") {
            let cmd_status = Command::new("REG")
                .arg("ADD")
                .arg("HKCU\\Environment")
                .arg("/v")
                .arg("PATH")
                .arg("/t")
                .arg(if path.contains(";%") { "REG_EXPAND_SZ" } else { "REG_SZ" })
                .arg("/d")
                .arg(format!("{}%REXE_HOME%;", path))
                .arg("/f")
                .stdout(Stdio::null())
                .status()?;
            assert!(cmd_status.success());
        }
    }

    println!(
        "Add '{}' to the PATH environment variable successfully!",
        curr_exe_path.to_str().unwrap()
    );

    Ok(())
}

pub(crate) fn set_alias(alias_name: &str) -> Result<(), Box<dyn Error>> {
    let curr_exe_path = get_curr_exe_path()?;
    seeval!(curr_exe_path);

    let rexe_home = curr_exe_path.as_path().parent().unwrap();
    seeval!(rexe_home);

    let curr_exe_name = curr_exe_path.as_path().file_name().unwrap();
    seeval!(curr_exe_name);

    let curr_dir = current_dir()?;

    let mut _alias_name = alias_name.to_string();
    #[cfg(windows)]
    if !_alias_name.ends_with(".exe") {
        _alias_name.push_str(".exe");
    };

    if !rexe_home.join(&_alias_name).exists() {
        std::env::set_current_dir(Path::new(rexe_home))?;
        #[cfg(windows)]
        let symlink_ret = {
            use std::os::windows::fs;
            fs::symlink_file(curr_exe_name, &_alias_name)
        };

        #[cfg(any(unix, target_os = "hermit"))]
        let symlink_ret = {
            use std::os::unix::fs;
            fs::symlink(curr_exe_name, &_alias_name)
        };

        if let Ok(_) = symlink_ret {
            println!(
                "Set the alias '{}' successfully.",
                &_alias_name[..(_alias_name.rfind(".").unwrap_or(_alias_name.len()))]
            );
        } else {
            eprintln!("Sorry, you don't have enough permissions to do this.");
            eprintln!("You can try again as an administrator or super user.");
        }
        std::env::set_current_dir(curr_dir)?;
    }

    Ok(())
}
