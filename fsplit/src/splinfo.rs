use crate::cli::ARGS;

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub(crate) static ref SPLIT_INFO_JSON_PATH: PathBuf = get_split_info_json_path();
    pub(crate) static ref SPLIT_INFO_JSON: &'static str = {
        let mut split_info_json_file = File::open(&*SPLIT_INFO_JSON_PATH).unwrap();
        let mut split_info_json = String::new();
        split_info_json_file.read_to_string(&mut split_info_json).unwrap_or(0usize);
        Box::leak(split_info_json.into_boxed_str())
    };
    pub(crate) static ref SPLIT_INFOS: Vec<SplitInfo<'static>> = {
        if let Ok(ret) = serde_json::from_str::<Vec<SplitInfo<'static>>>(*SPLIT_INFO_JSON) {
            ret
        } else {
            Vec::<SplitInfo<'static>>::new()
        }
    };
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub(crate) struct SplitInfo<'a> {
    pub(crate) file_name: &'a str,
    pub(crate) file_sha512sum: &'a str,
    pub(crate) block_paths: Vec<&'a str>,
}

impl<'a> SplitInfo<'a> {
    pub(crate) fn default() -> Self {
        SplitInfo { file_name: "", file_sha512sum: "", block_paths: Vec::<&'static str>::new() }
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
