use bytes::{Buf, Bytes};
use calamine::{open_workbook_auto, DataType, Error, Range, Reader, Sheets};

use std::env::{args_os, current_dir, temp_dir};
use std::ffi::OsString;
use std::fs::{create_dir_all, remove_file, File};
use std::io::{stdin, stdout, BufRead, BufReader, Write};
use std::path::{Path, MAIN_SEPARATOR};

use tokio::runtime::Runtime;

mod one_case;
use one_case::OneCase;

const ROBOT_COMMENT_EOL: &'static str = "\n    ";

fn fmt_robot_comment_lines(from: &str) -> String {
    let mut ret = String::new();
    let lines: Vec<&str> = from.split('\n').collect();
    let lines_len = lines.len();
    for i in 0..lines_len {
        let line = lines[i];
        if !line.is_empty() {
            ret.push_str("# ");
            ret.push_str(line.trim_end());
            if i != lines_len - 1 {
                ret.push_str(&ROBOT_COMMENT_EOL);
            }
        }
    }
    ret
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
        author_tag
            .get(0..(author_tag.len() - 1))
            .unwrap_or_default()
            .to_string(),
        mod_tag
            .get(0..(mod_tag.len() - 1))
            .unwrap_or_default()
            .to_string(),
    ))
}

async fn dl_xlsx(url: &str) -> Result<Bytes, Box<dyn std::error::Error>> {
    let resp_bytes = reqwest::get(url).await?.bytes().await?;
    Ok(resp_bytes)
}

fn sync_dl_xlsx(url: &str) -> Option<BufReader<bytes::buf::Reader<Bytes>>> {
    let tokio_rt = Runtime::new().unwrap();
    let ret_bytes = tokio_rt.block_on(dl_xlsx(url));
    if let Ok(bytes) = ret_bytes {
        return Some(BufReader::new(bytes.reader()));
    }
    None
}

fn open_workbook_from_http_link(url: &str) -> Result<Sheets, Error> {
    let tmp_dir = temp_dir();
    let default_path = tmp_dir.join(format!("{}{}", ".tmp", &url[(url.rfind(".").unwrap())..]));
    let mut tmp_xlsx = File::create(&default_path)?;
    let mut xlsx_reader = sync_dl_xlsx(url).unwrap();
    std::io::copy(&mut xlsx_reader, &mut tmp_xlsx)?;
    let result = open_workbook_auto(&default_path)?;
    remove_file(&default_path)?;
    Ok(result)
}

fn main() -> Result<(), Error> {
    let args_vec = args_os().collect::<Vec<OsString>>();

    let arg_1 = &args_vec[1];
    let arg_1_string = &arg_1.clone().into_string().unwrap();
    let mut workbook = if arg_1_string.starts_with("http") {
        open_workbook_from_http_link(&arg_1_string)?
    } else {
        open_workbook_auto(Path::new(&arg_1))?
    };

    let (author_tag, mod_tag) = get_author_and_mod_tag()?;

    if let Some(default_sheet) = default_sheet_of_wb(&mut workbook) {
        let one_case_field_names = OneCase::field_names();
        let one_case_field_names_len = one_case_field_names.len();
        let default_sheet_rows_len = default_sheet.rows().len();

        let mut json = String::new();
        json.push('[');
        for row_idx in 1..default_sheet_rows_len {
            json.push_str("{");
            for field_idx in 0..one_case_field_names_len {
                if let Some(one_case_field_name) = one_case_field_names.get(field_idx) {
                    let r#str = match read_row(&default_sheet, row_idx)[field_idx].get_string() {
                        Some(r#str) => r#str,
                        None => "",
                    };

                    json.push('"');
                    json.push_str(one_case_field_name);
                    json.push_str("\":\"");
                    json.push_str(
                        &r#str
                            .replace("\\", "\\\\")
                            .replace("\"", "\\\"")
                            .replace("\r", "")
                            .replace("\n", "\\n")
                            .replace("\t", "\\t")
                            .to_string(),
                    );
                    json.push('"');

                    if field_idx != one_case_field_names_len - 1 {
                        json.push(',');
                    }
                }
            }
            json.push('}');
            if row_idx != default_sheet_rows_len - 1 {
                json.push(',');
            }
        }

        json.push(']');

        let deserialized: Vec<OneCase> = serde_json::from_str(&json).unwrap();
        for mut one_case in deserialized {
            if one_case.test_methods.starts_with("自动化")
                && one_case.can_be_automated.starts_with("否")
            {
                one_case.feature_name = one_case
                    .feature_name
                    .replace('/', &MAIN_SEPARATOR.to_string());
                let case_dir = current_dir().unwrap().join(&one_case.feature_name);
                if !case_dir.exists() {
                    create_dir_all(&case_dir)?;
                }
                let robot_path = case_dir.join(String::from(&one_case.case_title) + ".robot");
                if !robot_path.exists() {
                    let mut robot_file = File::create(&robot_path)?;

                    let mut robot_template = String::from(include_str!("case.robot"));
                    robot_template = robot_template.replace("{{caseTitle}}", &one_case.case_title);
                    robot_template = robot_template.replace("{{caseId}}", &one_case.case_id);
                    robot_template =
                        robot_template.replace("{{useCaseLevel}}", &one_case.use_case_level);

                    if !&one_case.preconditions.is_empty() {
                        robot_template = robot_template.replace(
                            "{{preconditions}}",
                            &("# 前置条件".to_owned()
                                + ROBOT_COMMENT_EOL
                                + &fmt_robot_comment_lines(&one_case.preconditions)),
                        );
                    } else {
                        robot_template = robot_template.replace("{{preconditions}}", "");
                    }

                    robot_template = robot_template.replace(
                        "{{steps}}",
                        &("# 步骤".to_owned()
                            + ROBOT_COMMENT_EOL
                            + &fmt_robot_comment_lines(&one_case.steps)),
                    );

                    robot_template = robot_template.replace(
                        "{{desiredResult}}",
                        &("# 期望结果".to_owned()
                            + ROBOT_COMMENT_EOL
                            + &fmt_robot_comment_lines(&one_case.desired_result)),
                    );

                    if !&one_case.notes.is_empty() {
                        robot_template = robot_template.replace(
                            "{{notes}}",
                            &("# 备注".to_owned()
                                + ROBOT_COMMENT_EOL
                                + &fmt_robot_comment_lines(&one_case.notes)),
                        );
                    } else {
                        robot_template = robot_template.replace("{{notes}}", "");
                    }

                    if !&one_case.postcondition.is_empty() {
                        robot_template = robot_template.replace(
                            "{{postcondition}}",
                            &("# 后置条件".to_owned()
                                + ROBOT_COMMENT_EOL
                                + &fmt_robot_comment_lines(&one_case.postcondition)),
                        );
                    } else {
                        robot_template = robot_template.replace("{{postcondition}}", "");
                    }

                    if !&author_tag.is_empty() {
                        robot_template = robot_template.replace("UnNamedAuthor", &author_tag);
                    }

                    if !&mod_tag.is_empty() {
                        robot_template = robot_template.replace("UnNamedModule", &mod_tag);
                    }

                    robot_file.write_all(robot_template.as_bytes())?;
                    robot_file.flush()?;
                }
            }
        }

        // write_json(&mut json)?;
    }

    Ok(())
}

fn default_sheet_of_wb(wb: &mut impl Reader) -> Option<Range<DataType>> {
    if let Some(range_ret) = wb.worksheet_range_at(0) {
        if let Ok(default_sheet) = range_ret {
            return Some(default_sheet);
        }
    }
    None
}

#[allow(dead_code)]
fn sheet_hedaers_from(sheet: &Range<DataType>) -> Vec<String> {
    let mut headers = Vec::<String>::new();
    for header in read_row(sheet, 0) {
        if let Some(r#str) = header.get_string() {
            headers.push(r#str.to_string());
        }
    }
    headers
}

fn read_row(sheet: &Range<DataType>, idx: usize) -> Vec<&DataType> {
    let mut ret = Vec::<&DataType>::new();
    if let Some(row) = sheet.rows().nth(idx) {
        for cell in row {
            ret.push(cell);
        }
    }
    ret
}

#[allow(dead_code)]
fn write_json(json: &mut String) -> std::io::Result<()> {
    use std::fs::OpenOptions;

    let pwd = current_dir().unwrap();
    let path = pwd.join("tests").join("tmp.json");
    let mut tmp_json = OpenOptions::new().read(true).write(true).open(path)?;

    tmp_json.write_all(&mut json.as_bytes())?;
    tmp_json.flush()?;
    Ok(())
}
