mod cli;
mod sha512sum;
mod splinfo;

use lazy_static::__Deref;

use crate::cli::ARGS;
use crate::splinfo::{SplitInfo, SPLIT_INFOS, SPLIT_INFO_JSON};

use std::{
    fs::{File, OpenOptions},
    io::{Read, Result, Seek, SeekFrom, Write},
    marker::PhantomData,
    path::{Path, PathBuf},
};

#[inline]
fn write_block(file_path: &Path, part_bytes: &Vec<u8>, split_infos: &mut SplitInfo) -> Result<()> {
    let block_sha512sum = sha512sum::compute(&*part_bytes);
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
    // split_infos.block_paths.push((&block_file_path.to_string_lossy()).to_string());
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
    dbg!(file_sha512sum);
    let metadata = file_path.metadata()?;
    dbg!(&metadata.len());
    let block_size = args.block_size;
    let parts = metadata.len() / block_size;
    let mut seek = 0;
    // let split_info_json_path = get_split_info_json_path(&file_path)?;
    // let mut split_info_json_file = OpenOptions::new()
    //     .create(true)
    //     .truncate(true)
    //     .read(true)
    //     .write(true)
    //     .open(&split_info_json_path)?;

    let mut def_split_infos = vec![SplitInfo::<'static>::default()];
    let mut split_infos = if let Ok(ref de) = &*SPLIT_INFOS {
        panic!("0");
    } else {
        panic!("1");
    };
    let mut split_info = SplitInfo::default();
    // split_info.file_name = file_path.file_name().unwrap().to_string_lossy().to_string().clone();
    for _ in 0..parts {
        block_stream(&file_path, &mut seek, &block_size, &mut split_info)?;
    }
    end_block_stream(&file_path, &mut seek, &mut split_info)?;
    // (*split_infos).push(split_info);
    dbg!(parts);
    dbg!(seek);
    Ok(())
}
