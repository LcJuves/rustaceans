use crate::cli::ARGS;

use std::fs::File;
use std::io::{Read, Result};
use std::path::PathBuf;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub(crate) static ref SPLIT_INFO_JSON_PATH: PathBuf = get_split_info_json_path();
    pub(crate) static ref SPLIT_INFO_JSON: &'static str =
        Box::leak(read_split_info_json(&*SPLIT_INFO_JSON_PATH).unwrap().into_boxed_str());
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
    split_info_json_path_from(PathBuf::from(&args.file_path).parent().unwrap().to_path_buf())
}

pub(crate) fn split_info_json_path_from(path: PathBuf) -> PathBuf {
    let sijp = if path.is_dir() { path.join(".split_info.json") } else { path };
    if !sijp.exists() {
        File::create(&sijp).unwrap();
    }
    sijp
}

pub(crate) fn read_split_info_json(path: &PathBuf) -> Result<String> {
    let mut split_info_json_file = File::open(path)?;
    let mut split_info_json = String::new();
    split_info_json_file.read_to_string(&mut split_info_json)?;
    Ok(split_info_json)
}

// pub(crate) fn read_split_info(path: &PathBuf) -> Result<Vec<SplitInfo<'_>>> {
//     let mut ret = Vec::<SplitInfo<'_>>::new();
//     if let Ok(serde_ret) = serde_json::from_str::<Vec<SplitInfo<'_>>>(&read_split_info_json(path)?)
//     {
//         for i in serde_ret {
//             ret.push(i);
//         }
//     }
//     Ok(ret)
// }
