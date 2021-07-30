/**
 * Created at 2021/7/30 23:44
 *
 * @author Liangcheng Juves
 */
use lazy_static::lazy_static;

lazy_static! {
    pub static ref NOT_FOUND_TEMP_HTML: &'static str = include_str!("not_found_temp.html");
    pub static ref CRLF: &'static str = "\r\n";
    pub static ref LF: &'static str = "\n";
    pub static ref SP: &'static str = " ";
}
