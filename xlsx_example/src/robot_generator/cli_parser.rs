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
