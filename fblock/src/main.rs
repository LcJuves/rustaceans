mod blockopt;
mod cli;
mod hash;
mod splinfo;

use crate::blockopt::*;
use crate::cli::{Commands, ARGS_CMD};
use crate::splinfo::{SplitInfo, SPLIT_INFOS, SPLIT_INFO_JSON_PATH};

use std::{
    fs::OpenOptions,
    io::{Result, Write},
    path::Path,
};

fn main() -> Result<()> {
    let Commands::Split { file_path, block_size, .. } = &*ARGS_CMD;
    let file_path = Path::new(&file_path);
    let file_sha512sum = hash::compute(&std::fs::read(&file_path)?);
    #[cfg(debug_assertions)]
    dbg!(&file_sha512sum);
    let metadata = file_path.metadata()?;
    #[cfg(debug_assertions)]
    dbg!(&metadata.len());
    let block_size = block_size;
    let parts = metadata.len() / block_size;
    let mut seek = 0;

    let mut split_infos = SPLIT_INFOS.clone();
    #[cfg(debug_assertions)]
    dbg!(&split_infos);
    let mut split_info = SplitInfo::default();
    split_info.file_name =
        Box::leak(file_path.file_name().unwrap().to_string_lossy().to_string().into_boxed_str());
    split_info.file_sha512sum = Box::leak(file_sha512sum.into_boxed_str());
    for _ in 0..parts {
        block_stream(&file_path, &mut seek, &block_size, &mut split_info)?;
    }
    end_block_stream(&file_path, &mut seek, &mut split_info)?;
    #[cfg(debug_assertions)]
    dbg!(parts);
    #[cfg(debug_assertions)]
    dbg!(seek);
    split_infos.push(split_info);
    split_infos.dedup_by(|a, b| a.file_sha512sum == b.file_sha512sum);
    let mut split_info_json_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .read(true)
        .write(true)
        .open(&*SPLIT_INFO_JSON_PATH)?;
    split_info_json_file
        .write_all(serde_json::to_string_pretty(&split_infos).unwrap().as_bytes())?;
    split_info_json_file.flush()?;
    std::fs::remove_file(file_path)?;
    Ok(())
}
