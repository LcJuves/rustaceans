use crate::robot_generator::cli_parser::*;

use lazy_static::lazy_static;

lazy_static! {
    pub(crate) static ref ROBOT_COMMENT_EOL: &'static str = "\n    ";
    pub(crate) static ref ONE_LINE_COMMENT_PREFIX: &'static str = {
        if args_os_has_flag("--log-comments") {
            "Log    "
        } else {
            "# "
        }
    };
}

pub(crate) fn fmt_robot_comment_lines(from: &str) -> String {
    let mut ret = String::new();
    let lines: Vec<&str> = from.split('\n').collect();
    let lines_len = lines.len();
    for i in 0..lines_len {
        let mut line = lines[i].trim_end();
        if line.trim().is_empty() && i > 0 {
            continue;
        }
        ret.push_str(&ONE_LINE_COMMENT_PREFIX);
        if args_os_has_flag("--log-comments") {
            line = line.trim_start();
        }
        ret.push_str(line);
        if i != lines_len - 1 {
            ret.push_str(&ROBOT_COMMENT_EOL);
        }
    }
    ret
}
