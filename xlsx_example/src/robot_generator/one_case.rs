use crate::reflection::Reflection;
use crate::robot_generator::cli_parser::*;
use crate::robot_generator::robot_util::*;

use std::fs::{create_dir_all, OpenOptions};
use std::io::{stdin, stdout, BufRead, Write};
use std::path::{Path, MAIN_SEPARATOR};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref AUTHOR_AND_MOD_TAG: (String, String) =
        get_author_and_mod_tag().unwrap_or(("".to_string(), "".to_string()));
    static ref ROBOT_TEMPLATE: String = String::from(include_str!("case.robot"));
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OneCase {
    feature_name: String,
    case_id: String,
    case_title: String,
    preconditions: String,
    steps: String,
    postcondition: String,
    desired_result: String,
    test_methods: String,
    use_case_type: String,
    can_be_automated: String,
    tag: String,
    author: String,
    product_requirement_id: String,
    online_question_id: String,
    test_experience_id: String,
    use_case_level: String,
    notes: String,
}

impl Reflection for OneCase {
    fn field_names() -> Vec<String> {
        vec![
            "feature_name".to_string(),
            "case_id".to_string(),
            "case_title".to_string(),
            "preconditions".to_string(),
            "steps".to_string(),
            "postcondition".to_string(),
            "desired_result".to_string(),
            "test_methods".to_string(),
            "use_case_type".to_string(),
            "can_be_automated".to_string(),
            "tag".to_string(),
            "author".to_string(),
            "product_requirement_id".to_string(),
            "online_question_id".to_string(),
            "test_experience_id".to_string(),
            "use_case_level".to_string(),
            "notes".to_string(),
        ]
    }
}

impl OneCase {
    pub fn save_robot_to(&mut self, dir: &Path) -> std::io::Result<()> {
        let (ref author_tag, mod_tag) = &*AUTHOR_AND_MOD_TAG;

        if self.test_methods.starts_with("自动化") && self.can_be_automated.starts_with("否") {
            self.feature_name = self.feature_name.replace('/', &MAIN_SEPARATOR.to_string());
            let case_dir = &dir.join(&self.feature_name);
            if !case_dir.exists() {
                create_dir_all(&case_dir)?;
            }
            let robot_path = case_dir.join(format!("{}{}", &self.case_title, ".robot"));
            let overwritten_and_confirm_by_user = || -> std::io::Result<bool> {
                if !args_os_has_flag("--overwritten") {
                    return Ok(false);
                }

                stdout().write_all(
                    format!("Overwritten '{}' ? [y/N] ", &robot_path.to_string_lossy()).as_bytes(),
                )?;
                stdout().flush()?;
                let mut confirmation = String::new();
                stdin().lock().read_line(&mut confirmation)?;

                if confirmation.starts_with("y") || confirmation.starts_with("Y") {
                    return Ok(true);
                }

                Ok(false)
            };
            if overwritten_and_confirm_by_user().unwrap_or(false)
                || args_os_has_flag("--overwritten-slient")
                || !robot_path.exists()
            {
                if args_os_has_flag("-v") || args_os_has_flag("--verbose") {
                    println!("Generating {}", &robot_path.to_string_lossy());
                }
                let mut robot_file = OpenOptions::new()
                    .create(true)
                    .read(true)
                    .write(true)
                    .truncate(true)
                    .open(&robot_path)?;

                let mut robot_template = ROBOT_TEMPLATE.clone();
                robot_template = robot_template.replace("{{case_title}}", &self.case_title);
                robot_template = robot_template.replace("{{case_id}}", &self.case_id);
                robot_template = robot_template.replace("{{use_case_level}}", &self.use_case_level);

                robot_template = robot_template.replace(
                    "{{preconditions}}",
                    &("# 前置条件".to_owned()
                        + &ROBOT_COMMENT_EOL
                        + &fmt_robot_comment_lines(&self.preconditions)),
                );

                robot_template = robot_template.replace(
                    "{{steps}}",
                    &("# 步骤".to_owned()
                        + &ROBOT_COMMENT_EOL
                        + &fmt_robot_comment_lines(&self.steps)),
                );

                robot_template = robot_template.replace(
                    "{{desired_result}}",
                    &("# 期望结果".to_owned()
                        + &ROBOT_COMMENT_EOL
                        + &fmt_robot_comment_lines(&self.desired_result)),
                );

                if !&self.notes.is_empty() {
                    robot_template = robot_template.replace(
                        "{{notes}}",
                        &("# 备注".to_owned()
                            + &ROBOT_COMMENT_EOL
                            + &fmt_robot_comment_lines(&self.notes)),
                    );
                } else {
                    robot_template = robot_template.replace("{{notes}}", "");
                }

                if !&self.postcondition.is_empty() {
                    robot_template = robot_template.replace(
                        "{{postcondition}}",
                        &("# 后置条件".to_owned()
                            + &ROBOT_COMMENT_EOL
                            + &fmt_robot_comment_lines(&self.postcondition)),
                    );
                } else {
                    robot_template = robot_template.replace("{{postcondition}}", "");
                }

                if !&author_tag.trim().is_empty() {
                    robot_template = robot_template.replace("{{author_tag}}", &author_tag);
                } else {
                    robot_template = robot_template.replace("{{author_tag}}", "UnNamedAuthor");
                }

                if !&mod_tag.trim().is_empty() {
                    robot_template = robot_template.replace("{{mod_tag}}", &mod_tag);
                } else {
                    robot_template = robot_template.replace("{{mod_tag}}", "UnNamedModule");
                }

                robot_file.write_all(robot_template.as_bytes())?;
                robot_file.flush()?;
            }
        }

        Ok(())
    }
}

fn get_author_and_mod_tag() -> std::io::Result<(String, String)> {
    stdout().write(b"Please enter author tag: ")?;
    stdout().flush()?;
    let mut author_tag = String::new();
    stdin().lock().read_line(&mut author_tag)?;

    stdout().write(b"Please enter module tag: ")?;
    stdout().flush()?;
    let mut mod_tag = String::new();
    stdin().lock().read_line(&mut mod_tag)?;

    Ok((
        author_tag[..(author_tag
            .rfind("\r")
            .unwrap_or(author_tag.rfind("\n").unwrap()))]
            .to_string(),
        mod_tag[..(mod_tag.rfind("\r").unwrap_or(mod_tag.rfind("\n").unwrap()))].to_string(),
    ))
}
