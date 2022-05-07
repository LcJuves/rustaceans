use std::{
    fs::File,
    io::{copy, Read, Result, Seek, SeekFrom, Write},
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

fn main() -> Result<()> {
    let args = Args::parse();
    let file_path = Path::new(&args.file_path);
    let metadata = file_path.metadata()?;
    let block_size = args.block_size;
    let parts = metadata.len() / block_size;
    let mut seek = 0;
    for _ in 0..parts {
        let mut file = File::open(&file_path)?;
        let mut part_bytes = vec![0u8; block_size as usize];
        file.seek(SeekFrom::Start(seek))?;
        file.read_exact(&mut part_bytes)?;
        let block_sha512sum = compute_sha512sum(&part_bytes);
        println!("{}", block_sha512sum);
        seek += block_size;
        let mut block_file = File::create(&file_path.parent().unwrap().join(block_sha512sum))?;
        block_file.write_all(&part_bytes)?;
        block_file.flush()?;
    }
    println!("{}", parts);
    Ok(())
}
