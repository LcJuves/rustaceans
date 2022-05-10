mod cli;
mod sha512sum;
mod splinfo;

use crate::cli::ARGS;
use crate::splinfo::{SplitInfo, SPLIT_INFOS, SPLIT_INFO_JSON_PATH};

use std::path::Path;
use std::{
    fs::{File, OpenOptions},
    io::{Read, Result, Seek, SeekFrom, Write},
};

#[inline]
fn write_block(file_path: &Path, part_bytes: &Vec<u8>, split_infos: &mut SplitInfo) -> Result<()> {
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

fn block_stream(
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

fn end_block_stream(file_path: &Path, seek: &mut u64, split_info: &mut SplitInfo) -> Result<()> {
    let mut file = File::open(&file_path)?;
    let mut part_bytes = Vec::<u8>::new();
    file.seek(SeekFrom::Start(*seek))?;
    file.read_to_end(&mut part_bytes)?;
    write_block(file_path, &part_bytes, split_info)?;
    *seek += part_bytes.len() as u64;
    Ok(())
}

fn main() -> Result<()> {
    let args = &ARGS;
    let file_path = Path::new(&args.file_path);
    let file_sha512sum = sha512sum::compute(&std::fs::read(&file_path)?);
    #[cfg(debug_assertions)]
    dbg!(&file_sha512sum);
    let metadata = file_path.metadata()?;
    #[cfg(debug_assertions)]
    dbg!(&metadata.len());
    let block_size = args.block_size;
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
    Ok(())
}
