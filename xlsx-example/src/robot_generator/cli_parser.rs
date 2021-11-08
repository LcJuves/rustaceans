use std::env::args_os;
use std::ffi::OsString;

use clap::{App, Arg, ArgMatches};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CLI_MATCHES: ArgMatches<'static> = {
        App::new("RF TestCase Generator")
        .version("1.0")
        .author("Liangcheng Juves <liangchengj@outlook.com>")
        .usage("genrobot [FLAGS] [OPTIONS]")
        .arg(
            Arg::with_name("xlsx-url")
                .long("xlsx-url")
                .help("Set an xlsx url to generate robot files (priority is higher than `--xlsx-path` option)")
                .value_name("XLSX_URL")
                .takes_value(true)
                .required_unless("xlsx-path"),
        )
        .arg(
            Arg::with_name("xlsx-path")
                .long("xlsx-path")
                .help("Set an xlsx path to generate robot files")
                .value_name("XLSX_PATH")
                .takes_value(true)
                .required_unless("xlsx-url"),
        )
        .arg(
            Arg::with_name("author-tag")
                .long("author-tag")
                .help("The author tag is included in robot files to be generated")
                .value_name("AUTHOR_TAG")
                .takes_value(true)
                .required(false)
        )
        .arg(
            Arg::with_name("mod-tag")
                .long("mod-tag")
                .help("The module tag is included in robot files to be generated")
                .value_name("MOD_TAG")
                .takes_value(true)
                .required(false)
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Verbose"),
        )
        .arg(
            Arg::with_name("overwritten-slient")
                .long("overwritten-slient")
                .multiple(true)
                .help("If the robot file to be generated already exists, it will be overwritten"),
        )
        .arg(
            Arg::with_name("overwritten")
                .long("overwritten")
                .multiple(true)
                .help("If the robot file to be generated already exists, it will be overwritten (user confirmation is required)"),
        )
        .get_matches()
    };
}

pub(crate) fn args_os_has_flag(name: &str) -> bool {
    let args_vec = args_os().collect::<Vec<OsString>>();
    for arg in args_vec {
        if arg == name {
            return true;
        }
    }
    false
}

pub(crate) fn option_value_of(name: &str) -> Option<String> {
    let args_vec = args_os().collect::<Vec<OsString>>();
    for idx in 0..(args_vec.len() - 1) {
        if args_vec[idx] == name {
            if let Some(os_string) = args_vec.get(idx + 1usize) {
                let ret_string = os_string.to_string_lossy().to_string();
                return if !ret_string.trim().is_empty() {
                    Some(ret_string)
                } else {
                    None
                };
            } else {
                return None;
            }
        }
    }
    None
}
