use clap::{Parser, Subcommand};
use lazy_static::lazy_static;

lazy_static! {
    pub(crate) static ref ARGS: Args = {
        let args = Args::parse();
        args
    };
    pub(crate) static ref ARGS_CMD: Commands = {
        let args = &ARGS;
        (*args).command.clone()
    };
}

/// A tool for splitting and restoring files
#[derive(Parser, Debug)]
#[clap(propagate_version = true)]
#[clap(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[clap(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand, Debug)]
#[derive(Clone)]
pub(crate) enum Commands {
    /// Divide the file into equal chunks
    Split {
        /// The path of the file to be split
        #[clap(short, long)]
        file_path: String,

        /// The size of each block after splitting
        #[clap(long, default_value_t = 3145728)]
        block_size: u64,
        // /// JSON configuration file path for block description information
        // #[clap(long)]
        // back_from: String,
    },
}
