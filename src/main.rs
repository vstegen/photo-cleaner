use std::path::PathBuf;

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
    Clean {
        #[clap(short, long)]
        raw: PathBuf,
        #[clap(short, long)]
        compressed: PathBuf,
        #[clap(long)]
        dry: Option<bool>,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Clean {
            raw,
            compressed,
            dry: _,
        } => {
            println!(
                "raw path: {}, compressed path: {}",
                raw.display(),
                compressed.display()
            );
        }
    }
}
