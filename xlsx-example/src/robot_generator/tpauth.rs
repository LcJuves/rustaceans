use crate::robot_generator::tputil::*;

use std::error::Error;
use std::fs::File;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginedAuthInfo<'a> {
    ep_jwt_token_current: &'a str,
    sessionid: &'a str,
    user_name: &'a str,
    email: &'a str,
    staff_code: &'a str,
    token: &'a str,
}

impl<'a> LoginedAuthInfo<'a> {
    pub fn new() -> Self {
        LoginedAuthInfo {
            ep_jwt_token_current: "",
            sessionid: "",
            user_name: "",
            email: "",
            staff_code: "",
            token: "",
        }
    }

    pub fn ep_jwt_token_current(&self) -> String {
        self.ep_jwt_token_current.to_owned()
    }

    pub fn sessionid(&self) -> String {
        self.sessionid.to_owned()
    }

    pub fn user_name(&self) -> String {
        self.user_name.to_owned()
    }

    pub fn email(&self) -> String {
        self.email.to_owned()
    }

    pub fn staff_code(&self) -> String {
        self.staff_code.to_owned()
    }

    pub fn token(&self) -> String {
        self.token.to_owned()
    }
}

lazy_static! {
    pub static ref AUTH_CONF_JSON: std::io::Result<&'static str> = {
        if user_info_json_exist() {
            let user_info_json_path = USER_INFO_JSON_PATH.as_ref().unwrap();
            let mut user_info_json = File::open(&user_info_json_path)?;
            let mut user_info_json_bytes = Vec::<u8>::new();
            std::io::copy(&mut user_info_json, &mut user_info_json_bytes)?;
            let ret_string = (&String::from_utf8_lossy(&user_info_json_bytes)).to_string();
            return Ok(Box::leak(ret_string.into_boxed_str()));
        } else {
            println!("\u{1b}[91m{}\u{1b}[0m", "Please login first!");
            println!(
                "For more information try \u{1b}[92m{}\u{1b}[0m or \u{1b}[92m{}\u{1b}[0m",
                "--help", "-h"
            );
            std::process::exit(-1);
        }
    };
}
