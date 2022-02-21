/**
 * Created at 2021/7/30 23:44
 *
 * @author Liangcheng Juves
 */
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CRLF: &'static str = "\r\n";
    pub static ref LF: &'static str = "\n";
    pub(crate) static ref SP: &'static str = " ";
    pub(crate) static ref SERVER_NAME: &'static str = "rfserv";
    pub(crate) static ref RUST_LOGO_SVG: &'static str = include_str!("w3c/rust_logo.svg");
    pub(crate) static ref HTML_TEMP_PART1: &'static str = r#"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width,initial-scale=1"><meta http-equiv="X-UA-Compatible" content="ie=edge"><title>"#;
    pub(crate) static ref HTML_TEMP_PART2: &'static str = r#"</title><style>"#;
    pub(crate) static ref HTML_TEMP_PART3: &'static str =
        r#"</style></head><body><main id="main"></main><script>"#;
    pub(crate) static ref HTML_TEMP_PART4: &'static str = r#"</script></body></html>"#;
    pub(crate) static ref COMMON_CSS: &'static str = include_str!("w3c/common.min.css");
    pub(crate) static ref DIR_VIEWER_JS: &'static str = include_str!("w3c/dir_viewer.min.js");
    pub(crate) static ref NOT_FOUND_JS: &'static str = include_str!("w3c/not_found.min.js");
}

#[derive(Debug, Clone)]
pub(crate) struct FileInfo {
    pub(crate) name: String,
    pub(crate) last_modified: u128,
    pub(crate) size: u64,
    pub(crate) is_dir: bool,
}
