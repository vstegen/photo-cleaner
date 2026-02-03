use std::{
    fs,
    path::{Path, PathBuf},
    process,
};

use clap::{Parser, Subcommand};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[clap(name = "photo-cleanup")]
#[clap(arg_required_else_help = true)]
#[clap(version)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
struct CleanArgs {
    #[clap(short, long)]
    /// The directory in which the raw files can be found.
    raw: PathBuf,
    #[clap(short, long)]
    /// The directory in which the compressed files can be found.
    compressed: PathBuf,
    #[clap(long)]
    /// Do not delete files and instead output which files would be deleted.
    dry: bool,
    #[clap(short, long)]
    /// Print detailed output for each file operation.
    verbose: bool,
    #[clap(long)]
    /// Print only summary output (suppresses per-file logs and dry-run lists).
    summary_only: bool,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Deletes all JPEG images that have no matching RAW file.
    ///
    /// Matching files are identified by relative path and file name.
    Clean(CleanArgs),
    /// Deletes all JPEG images that do have a matching RAW file.
    ///
    /// Matching files are identified by relative path and file name.
    CleanMatched(CleanArgs),
}

#[derive(Clone, Copy, Debug)]
enum DeleteMode {
    Orphaned,
    Matched,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Command::Clean(clean_args) => {
            run_clean(clean_args, DeleteMode::Orphaned);
        }
        Command::CleanMatched(clean_args) => {
            run_clean(clean_args, DeleteMode::Matched);
        }
    }
}

fn run_clean(clean_args: CleanArgs, mode: DeleteMode) {
    let CleanArgs {
        raw,
        compressed,
        dry,
        verbose,
        summary_only,
    } = clean_args;

    if !raw.exists() {
        eprintln!("Error: Raw directory does not exist: {}", raw.display());
        process::exit(1);
    }
    if !raw.is_dir() {
        eprintln!("Error: Raw path is not a directory: {}", raw.display());
        process::exit(1);
    }
    if !compressed.exists() {
        eprintln!(
            "Error: Compressed directory does not exist: {}",
            compressed.display()
        );
        process::exit(1);
    }
    if !compressed.is_dir() {
        eprintln!(
            "Error: Compressed path is not a directory: {}",
            compressed.display()
        );
        process::exit(1);
    }

    clean_photos(&raw, &compressed, dry, verbose, summary_only, mode);
}

fn is_jpeg(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_lower = ext.to_string_lossy().to_lowercase();
        ext_lower == "jpg" || ext_lower == "jpeg"
    } else {
        false
    }
}

fn get_jpeg_files(compressed_root: &Path) -> Vec<PathBuf> {
    let mut jpeg_files = Vec::new();

    for entry in WalkDir::new(compressed_root)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() && is_jpeg(entry.path()) {
            jpeg_files.push(entry.path().to_path_buf());
        }
    }

    jpeg_files
}

fn find_matching_raw(
    compressed_file: &Path,
    compressed_root: &Path,
    raw_root: &Path,
) -> Option<PathBuf> {
    let relative_path = compressed_file.strip_prefix(compressed_root).ok()?;

    let parent_dir = relative_path.parent()?;
    let file_stem = compressed_file.file_stem()?;

    let raw_dir = raw_root.join(parent_dir);

    if !raw_dir.exists() {
        return None;
    }

    let raw_extensions = [
        "raf", "cr2", "cr3", "nef", "arw", "dng", "orf", "rw2", "raw",
    ];

    for ext in &raw_extensions {
        let potential_raw = raw_dir.join(format!("{}.{}", file_stem.to_string_lossy(), ext));
        if potential_raw.exists() {
            return Some(potential_raw);
        }
        let potential_raw_upper = raw_dir.join(format!(
            "{}.{}",
            file_stem.to_string_lossy(),
            ext.to_uppercase()
        ));
        if potential_raw_upper.exists() {
            return Some(potential_raw_upper);
        }
    }

    None
}

fn clean_photos(
    raw_root: &Path,
    compressed_root: &Path,
    dry_run: bool,
    verbose: bool,
    summary_only: bool,
    mode: DeleteMode,
) {
    println!(
        "Scanning for JPEG files in {}...",
        compressed_root.display()
    );

    let jpeg_files = get_jpeg_files(compressed_root);
    println!("Found {} JPEG files", jpeg_files.len());

    let mut to_delete = Vec::new();
    let mut errors = Vec::new();
    let mut matched_count = 0usize;

    for jpeg_file in &jpeg_files {
        match find_matching_raw(jpeg_file, compressed_root, raw_root) {
            Some(raw_file) => {
                matched_count += 1;
                if verbose && !summary_only {
                    println!("MATCH {} -> {}", jpeg_file.display(), raw_file.display());
                }
                if matches!(mode, DeleteMode::Matched) {
                    to_delete.push(jpeg_file.clone());
                }
            }
            None => {
                if verbose && !summary_only {
                    println!("NO_MATCH {}", jpeg_file.display());
                }
                if matches!(mode, DeleteMode::Orphaned) {
                    to_delete.push(jpeg_file.clone());
                }
            }
        }
    }

    let unmatched_count = jpeg_files.len().saturating_sub(matched_count);

    println!("\nSummary:");
    println!("  Total JPEG files: {}", jpeg_files.len());
    println!("  Files with matching RAW: {}", matched_count);
    println!("  Files without matching RAW: {}", unmatched_count);

    match mode {
        DeleteMode::Orphaned => println!("  Deleting orphaned JPEGs (no RAW)."),
        DeleteMode::Matched => println!("  Deleting matched JPEGs (has RAW)."),
    }

    if to_delete.is_empty() {
        match mode {
            DeleteMode::Orphaned => {
                println!("\nNo files to delete. All JPEGs have corresponding RAW files.");
            }
            DeleteMode::Matched => {
                println!("\nNo files to delete. No JPEGs have corresponding RAW files.");
            }
        }
        return;
    }

    if dry_run {
        if summary_only {
            println!(
                "\nDry run mode - {} files would be deleted.",
                to_delete.len()
            );
        } else {
            println!("\nDry run mode - files that would be deleted:");
            for file in &to_delete {
                println!("  {}", file.display());
            }
        }
    } else {
        println!("\nDeleting {} files...", to_delete.len());
        for file in &to_delete {
            match fs::remove_file(file) {
                Ok(_) => {
                    if verbose && !summary_only {
                        println!("  Deleted: {}", file.display());
                    }
                }
                Err(e) => {
                    eprintln!("  Error deleting {}: {}", file.display(), e);
                    errors.push(file.clone());
                }
            }
        }

        if !errors.is_empty() {
            eprintln!("\nEncountered {} errors during deletion", errors.len());
        } else {
            println!("\nSuccessfully deleted all {} files", to_delete.len());
        }
    }
}
