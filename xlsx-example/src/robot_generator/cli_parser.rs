use std::env::args_os;
use std::ffi::OsString;

use clap::{App, Arg, ArgMatches};
use lazy_static::lazy_static;

lazy_static! {
    pub(crate) static ref CLI_MATCHES: ArgMatches<'static> = init_cli_matches();
}

fn init_cli_matches() -> ArgMatches<'static> {
    App::new("RF TestCase Generator")
        .version("1.0.4")
        .author("Liangcheng Juves <liangchengj@outlook.com>")
        .usage("genrobot [FLAGS] [OPTIONS]")
        .arg(
            Arg::with_name("xlsx-url")
                .long("xlsx-url")
                .help("Set an xlsx url to generate robot files (priority is higher than `--xlsx-path` option)")
                .value_name("XLSX_URL")
                .takes_value(true)
                .required_unless_one(&vec!["xlsx-path","export-def-temp","tp-proj-name","tp-root-path","tp-path","upgrade","amtpv","set-alias","login"])
        )
        .arg(
            Arg::with_name("xlsx-path")
                .long("xlsx-path")
                .help("Set an xlsx path to generate robot files")
                .value_name("XLSX_PATH")
                .takes_value(true)
                .required_unless_one(&vec!["xlsx-url","export-def-temp","tp-proj-name","tp-root-path","tp-path","upgrade","amtpv","set-alias","login"])
        )
        .arg(
            Arg::with_name("author-tag")
                .long("author-tag")
                .help("The author tag is included in robot files to be generated")
                .value_name("AUTHOR_TAG")
                .takes_value(true)
                .required_unless_one(&vec!["xlsx-url","xlsx-path","export-def-temp","tp-proj-name","tp-root-path","tp-path","upgrade","amtpv","set-alias","login"])
        )
        .arg(
            Arg::with_name("mod-tag")
                .long("mod-tag")
                .help("The module tag is included in robot files to be generated")
                .value_name("MOD_TAG")
                .takes_value(true)
                .required_unless_one(&vec!["xlsx-url","xlsx-path","export-def-temp","tp-proj-name","tp-root-path","tp-path","upgrade","amtpv","set-alias","login"])
        )
        .arg(
            Arg::with_name("export-def-temp")
                .long("export-def-temp")
                .help("Export the default template used in the generated robot files")
                .value_name("EXPORT_PATH")
                .takes_value(true)
                .required(false)
        )
        .arg(
            Arg::with_name("use-temp")
                .long("use-temp")
                .help("Use the specified template to generate robot files")
                .value_name("TEMP_PATH")
                .takes_value(true)
                .required_unless_one(&vec!["xlsx-url","xlsx-path","export-def-temp","tp-proj-name","tp-root-path","tp-path","upgrade","amtpv","set-alias","login"])
        )
        .arg(
            Arg::with_name("out-dir")
                .long("out-dir")
                .help("Specify the storage path of the robot files to be generated")
                .value_name("DIR_NAME")
                .takes_value(true)
                .required_unless_one(&vec!["xlsx-url","xlsx-path","export-def-temp","tp-proj-name","tp-root-path","tp-path","upgrade","amtpv","set-alias","login"])
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .long("verbose")
                .multiple(true)
                .help("Verbose")
                .required_unless_one(&vec!["xlsx-url","xlsx-path","export-def-temp","tp-proj-name","tp-root-path","tp-path","upgrade","amtpv","set-alias","login"])
        )
        .arg(
            Arg::with_name("overwritten-slient")
                .long("overwritten-slient")
                .multiple(true)
                .help("If the robot file to be generated already exists, it will be overwritten")
                .required_unless_one(&vec!["xlsx-url","xlsx-path","export-def-temp","tp-proj-name","tp-root-path","tp-path","upgrade","amtpv","set-alias","login"])
        )
        .arg(
            Arg::with_name("overwritten")
                .long("overwritten")
                .multiple(true)
                .help("If the robot file to be generated already exists, it will be overwritten (user confirmation is required)")
                .required_unless_one(&vec!["xlsx-url","xlsx-path","export-def-temp","tp-proj-name","tp-root-path","tp-path","upgrade","amtpv","set-alias","login"])
        )
        .arg(
            Arg::with_name("log-comments")
                .long("log-comments")
                .multiple(true)
                .help("Use the `BuiltIn.Log` keyword to output comments in the robot file to be generated")
                .required_unless_one(&vec!["xlsx-url","xlsx-path","export-def-temp","tp-proj-name","tp-root-path","tp-path","upgrade","amtpv","set-alias","login"])
        )
        .arg(
            Arg::with_name("login")
                .long("login")
                .multiple(true)
                .help("Log in to the use case management platform")
        )
        // FIXME
        // .arg(
        //     Arg::with_name("upgrade")
        //         .long("upgrade")
        //         .multiple(true)
        //         .help("Upgrade executable to given version")
        // )
        // .arg(
        //     Arg::with_name("upgrade-host")
        //         .long("upgrade-host")
        //         .help("Upgrade from a server in the specified host (for example: `rexe-upgrade.io:9934`)")
        //         .value_name("DOMAIN")
        //         .takes_value(true)
        // )
        .arg(
            Arg::with_name("amtpv")
                .long("amtpv")
                .multiple(true)
                .help("Add the current execution file to the PATH environment variable")
        )
        .arg(
            Arg::with_name("set-alias")
                .long("set-alias")
                .help("Set an alias for the current executable file")
                .value_name("ALIAS_NAME")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("cond-js")
                .long("cond-js")
                .help("Use JavaScript to provide generation conditions (declared variables: `featureName`, `caseId`, `caseTitle`, `testMethods`, `useCaseType`, `canBeAutomated`, `tag`, `author`, `useCaseLevel`)")
                .value_name("JS_CONTENT")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("custom-js")
                .long("custom-js")
                .help("Use JavaScript to modify use case related information (declared variables: `featureName`, `caseId`, `caseTitle`, `testMethods`, `useCaseType`, `canBeAutomated`, `tag`, `author`, `useCaseLevel`)")
                .value_name("JS_CONTENT")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("tp-proj-name")
                .long("tp-proj-name")
                .help("Project name on the use case management platform")
                .value_name("PROJECT_NAME")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("tp-root-path")
                .long("tp-root-path")
                .help("The name of the root directory of the project on the use case management platform")
                .value_name("TP_ROOT_PATH")
                .takes_value(true)
        )
        .arg(
            Arg::with_name("tp-path")
                .long("tp-path")
                .help("The path under the root directory of the project on the use case management platform")
                .value_name("TP_PATH")
                .takes_value(true)
        )
        .get_matches()
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

pub(crate) fn option_value_of(name: &str) -> Option<OsString> {
    let args_vec = args_os().collect::<Vec<OsString>>();
    for idx in 0..(args_vec.len() - 1) {
        if args_vec[idx] == name {
            let os_string = args_vec.get(idx + 1usize).unwrap();
            return if !os_string.to_string_lossy().trim().is_empty() {
                Some(os_string.clone())
            } else {
                None
            };
        }
    }
    None
}
