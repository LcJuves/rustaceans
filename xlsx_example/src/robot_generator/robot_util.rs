use lazy_static::lazy_static;

lazy_static! {
    pub static ref ROBOT_COMMENT_EOL: &'static str = "\n    ";
}

pub(crate) fn fmt_robot_comment_lines(from: &str) -> String {
    let mut ret = String::new();
    let lines: Vec<&str> = from.split('\n').collect();
    let lines_len = lines.len();
    for i in 0..lines_len {
        ret.push_str("# ");
        ret.push_str(lines[i].trim_end());
        if i != lines_len - 1 {
            ret.push_str(&ROBOT_COMMENT_EOL);
        }
    }
    ret
}
