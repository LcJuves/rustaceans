use crate::cli::ARGS;

use std::fs::{File, OpenOptions};
use std::io::{Read, Result};
use std::path::{Path, PathBuf};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub(crate) static ref SPLIT_INFO_JSON_PATH: PathBuf = get_split_info_json_path();
    pub(crate) static ref SPLIT_INFO_JSON: Result<&'static str> = {
        let mut split_info_json_file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .read(true)
            .write(true)
            .open(&*SPLIT_INFO_JSON_PATH)?;
        let mut split_info_json = String::new();
        split_info_json_file.read_to_string(&mut split_info_json)?;
        Ok(Box::leak(split_info_json.into_boxed_str()))
    };
    pub(crate) static ref SPLIT_INFOS: Result<Vec<SplitInfo<'static>>> =
        Ok(serde_json::from_str::<Vec<SplitInfo<'static>>>(SPLIT_INFO_JSON.as_ref().unwrap())?);
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SplitInfo<'a> {
    pub(crate) file_name: &'a str,
    pub(crate) block_paths: Vec<&'a str>,
}

impl<'a> SplitInfo<'a> {
    pub(crate) fn default() -> Self {
        SplitInfo { file_name: "", block_paths: vec![""] }
    }
}

fn get_split_info_json_path() -> PathBuf {
    let args = &ARGS;
    let sijp = Path::new(&args.file_path).parent().unwrap().join(".split_info.json");
    if !sijp.exists() {
        File::create(&sijp).unwrap();
    }
    sijp
}
