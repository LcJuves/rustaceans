use std::{
    fs::{File, OpenOptions},
    io::{Read, Result, Seek, SeekFrom, Write},
    path::Path,
};

use clap::Parser;
use ssri::{Algorithm, IntegrityOpts};

fn compute_sha512sum(bytes: &[u8]) -> String {
    let integrity = IntegrityOpts::new().algorithm(Algorithm::Sha512).chain(bytes).result();
    let (_, hex) = integrity.to_hex();
    hex
}

/// A command line tool for splitting files
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The path of the file to be split
    #[clap(short, long)]
    file_path: String,

    /// The size of each block after splitting
    #[clap(long, default_value_t = 3145728)]
    block_size: u64,
}

#[inline]
fn write_block(file_path: &Path, part_bytes: &Vec<u8>) -> Result<()> {
    let block_sha512sum = compute_sha512sum(&*part_bytes);
    dbg!(&block_sha512sum);
    let mut block_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .read(true)
        .write(true)
        .open(file_path.parent().unwrap().join(block_sha512sum))?;
    block_file.write_all(&*part_bytes)?;
    block_file.flush()?;
    Ok(())
}

fn block_stream(file_path: &Path, seek: &mut u64, block_size: &u64) -> Result<()> {
    let mut file = File::open(&file_path)?;
    let mut part_bytes = vec![0u8; *block_size as usize];
    file.seek(SeekFrom::Start(*seek))?;
    file.read_exact(&mut part_bytes)?;
    write_block(file_path, &part_bytes)?;
    *seek += block_size;
    Ok(())
}

fn end_block_stream(file_path: &Path, seek: &mut u64) -> Result<()> {
    let mut file = File::open(&file_path)?;
    let mut part_bytes = Vec::<u8>::new();
    file.seek(SeekFrom::Start(*seek))?;
    file.read_to_end(&mut part_bytes)?;
    write_block(file_path, &part_bytes)?;
    *seek += part_bytes.len() as u64;
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    let file_path = Path::new(&args.file_path);
    let file_sha512sum = compute_sha512sum(&std::fs::read(&file_path)?);
    dbg!(file_sha512sum);
    let metadata = file_path.metadata()?;
    dbg!(&metadata.len());
    let block_size = args.block_size;
    let parts = metadata.len() / block_size;
    let mut seek = 0;
    for _ in 0..parts {
        block_stream(&file_path, &mut seek, &block_size)?;
    }
    end_block_stream(&file_path, &mut seek)?;
    dbg!(parts);
    dbg!(seek);
    Ok(())
}