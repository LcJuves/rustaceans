use crate::reflection::Reflection;
use crate::robot_generator::cli_parser::*;
use crate::robot_generator::util::*;
use crate::util::common::remove_eol;

use core::char::REPLACEMENT_CHARACTER;
use std::error::Error;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{stdin, stdout, BufRead, Write};
use std::path::{Path, MAIN_SEPARATOR};
use std::sync::Once;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref AUTHOR_AND_MOD_TAG: Result<(String, String), std::io::Error> =
        get_author_and_mod_tag();
    pub(crate) static ref ROBOT_TEMPLATE: String =
        String::from(include_str!("case.robot")).replace("\r\n", "\n");
    static ref USER_ROBOT_TEMPLATE: Result<String, std::io::Error> = read_user_robot_template();
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct OneCase {
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
            "feature_name",
            "case_id",
            "case_title",
            "preconditions",
            "steps",
            "postcondition",
            "desired_result",
            "test_methods",
            "use_case_type",
            "can_be_automated",
            "tag",
            "author",
            "product_requirement_id",
            "online_question_id",
            "test_experience_id",
            "use_case_level",
            "notes",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }
}

impl OneCase {
    const ONCE_INIT: Once = Once::new();

    fn replace_unsupport_char(&mut self) {
        let r#fn = |string: &str| -> String {
            string
                .replace(":", &REPLACEMENT_CHARACTER.to_string())
                .replace("*", &REPLACEMENT_CHARACTER.to_string())
                .replace("?", &REPLACEMENT_CHARACTER.to_string())
                .replace(r#"""#, &REPLACEMENT_CHARACTER.to_string())
                .replace("<", &REPLACEMENT_CHARACTER.to_string())
                .replace(">", &REPLACEMENT_CHARACTER.to_string())
                .replace("|", &REPLACEMENT_CHARACTER.to_string())
        };

        self.feature_name = r#fn(&self.feature_name);
        self.case_title = r#fn(&self.case_title);
    }

    fn overwritten_and_confirm_by_user(robot_path: &Path) -> Result<bool, std::io::Error> {
        if !args_os_has_flag("--overwritten") {
            return Ok(false);
        }

        stdout().write_all(
            format!("Overwritten '{}' ? [y/N] ", robot_path.to_string_lossy()).as_bytes(),
        )?;
        stdout().flush()?;
        let mut confirmation = String::new();
        stdin().lock().read_line(&mut confirmation)?;

        if confirmation.starts_with("y") || confirmation.starts_with("Y") {
            return Ok(true);
        }

        Ok(false)
    }

    fn expect_cond_js_return_true(&self) -> Result<bool, boa::JsValue> {
        if !args_os_has_flag("--cond-js") {
            return Ok(true);
        }

        if let Some(cond_js) = option_value_of("--cond-js") {
            let cond_js = cond_js.to_str().unwrap().to_owned();

            let mut jsctx = boa::Context::new();
            jsctx.eval(format!("let featureName='{}';", self.feature_name))?;
            jsctx.eval(format!("let caseId='{}';", self.case_id))?;
            jsctx.eval(format!("let caseTitle='{}';", self.case_title))?;
            jsctx.eval(format!("let testMethods='{}';", self.test_methods))?;
            jsctx.eval(format!("let useCaseType='{}';", self.use_case_type))?;
            jsctx.eval(format!("let canBeAutomated='{}';", self.can_be_automated))?;
            jsctx.eval(format!("let tag='{}';", self.tag))?;
            jsctx.eval(format!("let author='{}';", self.author))?;
            jsctx.eval(format!("let useCaseLevel='{}';", self.use_case_level))?;

            let cond_js_val = jsctx.eval(cond_js)?;
            if let boa::JsValue::Boolean(jsbool) = cond_js_val {
                return Ok(jsbool);
            }
        }

        Ok(true)
    }

    pub(crate) fn save_robot_to(&mut self, dir: &Path) -> Result<(), Box<dyn Error>> {
        Self::ONCE_INIT.call_once(|| {
            if !dir.exists() {
                if let None = dir.extension() {
                    if let Err(e) = create_dir_all(&dir) {
                        eprintln!("{}", e);
                    }
                }
            }

            if !dir.is_dir() {
                println!("Error option with --out-dir, is not a valid dir path");
                std::process::exit(-1);
            }
        });

        if self.can_be_automated.starts_with("否")
            && self.expect_cond_js_return_true().unwrap_or(true)
        {
            self.feature_name = self.feature_name.replace('/', &MAIN_SEPARATOR.to_string());
            self.case_title = self
                .case_title
                .replace(r"\", &REPLACEMENT_CHARACTER.to_string())
                .replace("/", &REPLACEMENT_CHARACTER.to_string());
            self.replace_unsupport_char();

            let case_dir = &dir.join(&self.feature_name);
            if !case_dir.exists() {
                create_dir_all(&case_dir)?;
            }
            let robot_path = case_dir.join(format!("{}{}", &self.case_title, ".robot"));
            if Self::overwritten_and_confirm_by_user(&robot_path)?
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

                let user_robot_template = USER_ROBOT_TEMPLATE.as_ref()?.replace("\r\n", "\n");
                let mut robot_template = if !user_robot_template.is_empty() {
                    user_robot_template.clone()
                } else {
                    ROBOT_TEMPLATE.clone()
                };
                robot_template = robot_template.replace("{{case_title}}", &self.case_title);
                robot_template = robot_template.replace("{{case_id}}", &self.case_id);
                robot_template = robot_template.replace("{{use_case_level}}", &self.use_case_level);

                robot_template = robot_template.replace(
                    "{{preconditions}}",
                    &(format!("{}前置条件", &*ONE_LINE_COMMENT_PREFIX).to_owned()
                        + &ROBOT_COMMENT_EOL
                        + &fmt_robot_comment_lines(&self.preconditions)),
                );

                robot_template = robot_template.replace(
                    "{{steps}}",
                    &(format!("{}步骤", &*ONE_LINE_COMMENT_PREFIX).to_owned()
                        + &ROBOT_COMMENT_EOL
                        + &fmt_robot_comment_lines(&self.steps)),
                );

                robot_template = robot_template.replace(
                    "{{desired_result}}",
                    &(format!("{}期望结果", &*ONE_LINE_COMMENT_PREFIX).to_owned()
                        + &ROBOT_COMMENT_EOL
                        + &fmt_robot_comment_lines(&self.desired_result)),
                );

                if !&self.notes.is_empty() {
                    robot_template = robot_template.replace(
                        "{{notes}}",
                        &(format!("{}备注", &*ONE_LINE_COMMENT_PREFIX).to_owned()
                            + &ROBOT_COMMENT_EOL
                            + &fmt_robot_comment_lines(&self.notes)),
                    );
                } else {
                    robot_template = robot_template.replace("{{notes}}", "");
                }

                if !&self.postcondition.is_empty() {
                    robot_template = robot_template.replace(
                        "{{postcondition}}",
                        &(format!("{}后置条件", &*ONE_LINE_COMMENT_PREFIX).to_owned()
                            + &ROBOT_COMMENT_EOL
                            + &fmt_robot_comment_lines(&self.postcondition)),
                    );
                } else {
                    robot_template = robot_template.replace("{{postcondition}}", "");
                }

                let (author_tag, mod_tag) = AUTHOR_AND_MOD_TAG.as_ref()?;

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

                if self.case_title.contains("二合一") {
                    robot_template = robot_template
                        .replace("${SdpConsoleUrl}", "${HyrBirdUrl}")
                        .replace("${SdpConsoleUser}", "${HyrBirdUser}")
                        .replace("${SdpConsolePasswd}", "${HyrBirdPasswd}");
                }

                robot_file.write_all(robot_template.as_bytes())?;
                robot_file.flush()?;
            }
        }

        Ok(())
    }
}

fn get_author_and_mod_tag() -> Result<(String, String), std::io::Error> {
    let mut author_tag = String::new();
    let mut mod_tag = String::new();

    if let Some(val) = option_value_of("--author-tag") {
        author_tag = val.to_string_lossy().trim().to_string();
    } else {
        stdout().write(b"Please enter author tag: ")?;
        stdout().flush()?;
        stdin().lock().read_line(&mut author_tag)?;
    }

    if let Some(val) = option_value_of("--mod-tag") {
        mod_tag = val.to_string_lossy().trim().to_string();
    } else {
        stdout().write(b"Please enter module tag: ")?;
        stdout().flush()?;
        stdin().lock().read_line(&mut mod_tag)?;
    }

    Ok((remove_eol(&author_tag), remove_eol(&mod_tag)))
}

fn read_user_robot_template() -> Result<String, std::io::Error> {
    if let Some(temp_path) = option_value_of("--use-temp") {
        let mut temp_file = File::open(Path::new(&temp_path))?;
        let mut ret_vec = Vec::<u8>::new();
        std::io::copy(&mut temp_file, &mut ret_vec)?;
        let temp_file_text = String::from_utf8_lossy(&ret_vec);
        for interpolation_expression in vec![
            "{{case_title}}",
            "{{case_id}}",
            "{{use_case_level}}",
            "{{preconditions}}",
            "{{steps}}",
            "{{desired_result}}",
            "{{notes}}",
            "{{postcondition}}",
            "{{author_tag}}",
            "{{mod_tag}}",
        ] {
            if !temp_file_text.contains(interpolation_expression) {
                println!(
                    "There is no interpolation expression in your template: {}",
                    interpolation_expression
                );
                std::process::exit(-1);
            }
        }
        return Ok(temp_file_text.to_string());
    }
    Ok(String::new())
}
