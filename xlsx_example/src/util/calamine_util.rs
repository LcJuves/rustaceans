use bytes::{Buf, Bytes};
use calamine::{open_workbook_auto, DataType, Error, Range, Reader, Sheets};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::env::{current_dir, temp_dir};
use std::fs::{remove_file, File};
use std::io::{BufReader, Write};
use tokio::runtime::Runtime;

use crate::reflection::Reflection;

lazy_static! {
    static ref TOKIO_RT: Runtime = Runtime::new().unwrap();
}

pub(crate) fn default_sheet_of_wb(wb: &mut impl Reader) -> Option<Range<DataType>> {
    if let Some(range_ret) = wb.worksheet_range_at(0) {
        if let Ok(default_sheet) = range_ret {
            return Some(default_sheet);
        }
    }
    None
}

#[allow(dead_code)]
pub(crate) fn sheet_hedaers_from(sheet: &Range<DataType>) -> Vec<String> {
    let mut headers = Vec::<String>::new();
    for header in read_row(sheet, 0) {
        if let Some(r#str) = header.get_string() {
            headers.push(r#str.to_string());
        }
    }
    headers
}

pub(crate) fn read_row(sheet: &Range<DataType>, idx: usize) -> Vec<&DataType> {
    let mut ret = Vec::<&DataType>::new();
    if let Some(row) = sheet.rows().nth(idx) {
        for cell in row {
            ret.push(cell);
        }
    }
    ret
}

async fn dl_excel(url: &str) -> Result<Bytes, Box<dyn std::error::Error>> {
    let resp_bytes = reqwest::get(url).await?.bytes().await?;
    Ok(resp_bytes)
}

fn sync_dl_excel(url: &str) -> Option<BufReader<bytes::buf::Reader<Bytes>>> {
    let ret_bytes = TOKIO_RT.block_on(dl_excel(url));
    if let Ok(bytes) = ret_bytes {
        return Some(BufReader::new(bytes.reader()));
    }
    None
}

pub(crate) fn open_workbook_by_url(url: &str) -> Result<Sheets, Error> {
    let tmp_dir = temp_dir();
    let default_path = tmp_dir.join(format!("{}{}", ".excel", &url[(url.rfind(".").unwrap())..]));
    let mut tmp_xlsx = File::create(&default_path)?;
    let mut xlsx_reader = sync_dl_excel(url).unwrap();
    std::io::copy(&mut xlsx_reader, &mut tmp_xlsx)?;
    let result = open_workbook_auto(&default_path)?;
    remove_file(&default_path)?;
    Ok(result)
}

pub(crate) fn sheet_to_json<'a, R: Reflection>(sheet: &'a Range<DataType>) -> &'a str {
    let reflection_field_names = R::field_names();
    let reflection_field_names_len = reflection_field_names.len();
    let sheet_rows_len = sheet.rows().len();

    let mut json = "".to_string();
    json.push('[');
    for row_idx in 1..sheet_rows_len {
        json.push_str("{");
        for field_idx in 0..reflection_field_names_len {
            if let Some(reflection_field_name) = reflection_field_names.get(field_idx) {
                let r#str = match read_row(&sheet, row_idx)[field_idx].get_string() {
                    Some(r#str) => r#str,
                    None => "",
                };

                json.push('"');
                json.push_str(reflection_field_name);
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

                if field_idx != reflection_field_names_len - 1 {
                    json.push(',');
                }
            }
        }
        json.push('}');
        if row_idx != sheet_rows_len - 1 {
            json.push(',');
        }
    }
    json.push(']');
    // write_json(&mut json)?;

    Box::leak(json.into_boxed_str()) /* unsafe */
}

pub(crate) fn sheet_reflect<'a, RD: Reflection + Deserialize<'a>>(
    sheet: &'a Range<DataType>,
) -> serde_json::error::Result<Vec<RD>> {
    let sheet_json = sheet_to_json::<RD>(sheet);
    let ret_vec: Vec<RD> = serde_json::from_str(&sheet_json)?;
    Ok(ret_vec)
}

#[allow(dead_code)]
fn write_json(json: &mut String) -> std::io::Result<()> {
    use std::fs::OpenOptions;

    let pwd = current_dir().unwrap();
    let path = pwd.join("tests").join("tmp.json");

    let mut tmp_json = OpenOptions::new()
        .create(true)
        .truncate(true)
        .read(true)
        .write(true)
        .open(&path)?;

    tmp_json.write_all(&mut json.as_bytes())?;
    tmp_json.flush()?;
    Ok(())
}
