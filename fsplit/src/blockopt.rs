use crate::cli::ARGS;
use crate::sha512sum;
use crate::splinfo::{split_info_json_path_from, SplitInfo, SPLIT_INFO_JSON_PATH};

use std::path::PathBuf;
use std::{
    fs::{File, OpenOptions},
    io::{Read, Result, Seek, SeekFrom, Write},
    path::Path,
};

#[inline]
pub(crate) fn write_block(
    file_path: &Path,
    part_bytes: &Vec<u8>,
    split_infos: &mut SplitInfo,
) -> Result<()> {
    let block_sha512sum = sha512sum::compute(&*part_bytes);
    #[cfg(debug_assertions)]
    dbg!(&block_sha512sum);
    let block_file_path = file_path.parent().unwrap().join(block_sha512sum);
    let mut block_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .read(true)
        .write(true)
        .open(&block_file_path)?;
    block_file.write_all(&*part_bytes)?;
    block_file.flush()?;
    let mut block_file_path_string = (&block_file_path.to_string_lossy()).to_string();
    #[cfg(debug_assertions)]
    dbg!(&block_file_path_string);
    let split_info_dir_path_string =
        (*SPLIT_INFO_JSON_PATH).parent().unwrap().to_string_lossy().to_string();
    #[cfg(windows)]
    let split_info_dir_path_string = split_info_dir_path_string.replace("\\", "/");
    #[cfg(debug_assertions)]
    dbg!(&split_info_dir_path_string);
    #[cfg(windows)]
    #[allow(unused_mut)]
    let mut block_file_path_string = block_file_path_string.replace("\\", "/");
    block_file_path_string =
        block_file_path_string.replace(&format!("{}{}", split_info_dir_path_string, "/"), "");
    #[cfg(debug_assertions)]
    dbg!(&block_file_path_string);
    split_infos.block_paths.push(Box::leak(block_file_path_string.into_boxed_str()));
    Ok(())
}

pub(crate) fn block_stream(
    file_path: &Path,
    seek: &mut u64,
    block_size: &u64,
    split_info: &mut SplitInfo,
) -> Result<()> {
    let mut file = File::open(&file_path)?;
    let mut part_bytes = vec![0u8; *block_size as usize];
    file.seek(SeekFrom::Start(*seek))?;
    file.read_exact(&mut part_bytes)?;
    write_block(file_path, &part_bytes, split_info)?;
    *seek += block_size;
    Ok(())
}

pub(crate) fn end_block_stream(
    file_path: &Path,
    seek: &mut u64,
    split_info: &mut SplitInfo,
) -> Result<()> {
    let mut file = File::open(&file_path)?;
    let mut part_bytes = Vec::<u8>::new();
    file.seek(SeekFrom::Start(*seek))?;
    file.read_to_end(&mut part_bytes)?;
    write_block(file_path, &part_bytes, split_info)?;
    *seek += part_bytes.len() as u64;
    Ok(())
}

pub(crate) fn blocks_to_file() -> Result<()> {
    let args = &ARGS;
    let sijp = split_info_json_path_from(PathBuf::from(&args.back_from));
    Ok(())
}
