use clap::Parser;
use lazy_static::lazy_static;

lazy_static! {
    pub(crate) static ref ARGS: Args = {
        let args = Args::parse();
        args
    };
}

/// A command line tool for splitting files
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Args {
    /// The path of the file to be split
    #[clap(short, long)]
    pub(crate) file_path: String,

    /// The size of each block after splitting
    #[clap(long, default_value_t = 3145728)]
    pub(crate) block_size: u64,

    /// JSON configuration file path for block description information
    #[clap(long)]
    pub(crate) back_from: String,
}
