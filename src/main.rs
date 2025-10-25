use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(name = "photo-cleanup")]
#[clap(arg_required_else_help = true)]
#[clap(version)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Deletes all JPEG images that have no matching RAW file.
    ///
    /// The raw files is assumed to have the RAW extension (Fujifilm).
    /// Matching files are identified by time taken and the file name.
    Clean {
        #[clap(short, long)]
        /// The directory in which the raw files can be found.
        raw: PathBuf,
        #[clap(short, long)]
        /// The directory in which the compressed files can be found.
        compressed: PathBuf,
        #[clap(long)]
        /// Do not delete files and instead output which files would be deleted.
        dry: Option<bool>,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Clean {
            raw,
            compressed: _,
            dry: _,
        } => if (!raw.is_dir()) {},
    }
}
