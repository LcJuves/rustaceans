use crate::robot_generator::tputil::*;
use crate::robot_generator::upgrade::get_curr_exe_path;
use crate::seeval;

use std::fs::File;
use std::io::{stdin, stdout, BufRead, Error, Write};
use std::process::Command;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct LoggedAuthInfo<'a> {
    ep_jwt_token_current: &'a str,
    sessionid: &'a str,
    user_name: &'a str,
    email: &'a str,
    staff_code: &'a str,
    token: &'a str,
}

#[allow(dead_code)]
impl<'a> LoggedAuthInfo<'a> {
    pub(crate) fn new() -> Self {
        LoggedAuthInfo {
            ep_jwt_token_current: "",
            sessionid: "",
            user_name: "",
            email: "",
            staff_code: "",
            token: "",
        }
    }

    pub(crate) fn ep_jwt_token_current(&self) -> String {
        self.ep_jwt_token_current.to_owned()
    }

    pub(crate) fn sessionid(&self) -> String {
        self.sessionid.to_owned()
    }

    pub(crate) fn user_name(&self) -> String {
        self.user_name.to_owned()
    }

    pub(crate) fn email(&self) -> String {
        self.email.to_owned()
    }

    pub(crate) fn staff_code(&self) -> String {
        self.staff_code.to_owned()
    }

    pub(crate) fn token(&self) -> String {
        self.token.to_owned()
    }
}

fn login_and_confirm_by_user() -> Result<bool, Error> {
    stdout().write_all(
        b"It is detected that you are not logged in, whether to log in immediately? [y/N] ",
    )?;
    stdout().flush()?;
    let mut confirmation = String::new();
    stdin().lock().read_line(&mut confirmation)?;

    if confirmation.starts_with("y") || confirmation.starts_with("Y") {
        return Ok(true);
    }

    Ok(false)
}

#[inline]
fn please_login() {
    println!("\x1b[91m{}\x1b[0m", "Please login first with option `--login`!");
    println!("For more information try \x1b[92m{}\x1b[0m or \x1b[92m{}\x1b[0m", "--help", "-h");
    std::process::exit(-1);
}

lazy_static! {
    pub(crate) static ref AUTH_CONF_JSON: Result<&'static str, Error> = {
        if !user_info_json_exist() {
            if login_and_confirm_by_user()? {
                let curr_exe_path = get_curr_exe_path()?;
                seeval!(curr_exe_path);
                let cmd_status = Command::new(curr_exe_path.as_os_str()).arg("--login").status()?;
                assert!(cmd_status.success());
            }
        }

        if !user_info_json_exist() {
            please_login();
        }

        let user_info_json_path = USER_INFO_JSON_PATH.as_ref().unwrap();
        let mut user_info_json = File::open(&user_info_json_path)?;
        let mut user_info_json_bytes = Vec::<u8>::new();
        std::io::copy(&mut user_info_json, &mut user_info_json_bytes)?;
        let ret_string = (&String::from_utf8_lossy(&user_info_json_bytes)).to_string();
        return Ok(Box::leak(ret_string.into_boxed_str()));
    };
    pub(crate) static ref AUTH_CONF: Result<LoggedAuthInfo<'static>, Error> = {
        let auth_conf_json = AUTH_CONF_JSON.as_ref().unwrap();
        Ok(serde_json::from_str(auth_conf_json)?)
    };
}
