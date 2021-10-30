use clap::{App, Arg, ArgMatches};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CLI_MATCHES: ArgMatches<'static> = {
        App::new("RF Robot Generator")
        .version("1.0")
        .author("Liangcheng Juves <liangchengj@outlook.com>")
        .usage("xlsx_example [FLAGS] [OPTIONS]")
        .arg(
            Arg::with_name("xlsx-url")
                .long("xlsx-url")
                .help("Set an xlsx url to generate robot files (priority is higher than `xlsx-path` option)")
                .takes_value(true)
                .required_unless("xlsx-path"),
        )
        .arg(
            Arg::with_name("xlsx-path")
                .long("xlsx-path")
                .help("Set an xlsx path to generate robot files")
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
