# photo-cleanup

photo-cleanup is a command-line tool to remove JPEG files based on whether a corresponding RAW file exists. It compares relative paths and file stems between two directory trees (one for RAW files and one for JPEGs) and then deletes JPEGs that either do or do not have a RAW match, depending on the selected mode.

## Features

- Deletes JPEG files with no matching RAW file (orphaned JPEGs).
- Deletes JPEG files that do have a matching RAW file (matched JPEGs).
- Supports dry-run mode and summary-only output.
- Works with common RAW extensions.

## How Matching Works

A JPEG file is considered a match if a RAW file exists with the same relative path and filename (stem) under the RAW root. For example:

- `jpeg/foo/bar/test.jpeg`
- `raw/foo/bar/test.raf`

These are considered a match because the relative path is the same and the filename stem is `test`.

## Supported Formats

- JPEG: `.jpg`, `.jpeg`
- RAW: `.raf`, `.cr2`, `.cr3`, `.nef`, `.arw`, `.dng`, `.orf`, `.rw2`, `.raw`

## Requirements

- Rust toolchain (for building from source)

## Installation

Build from source:

```bash
cargo build --release
```

The binary will be located at `target/release/photo-cleanup`.

## Usage

The CLI provides two subcommands:

- `clean`: Delete JPEG files without a matching RAW file.
- `clean-matched`: Delete JPEG files that do have a matching RAW file.

### Common Flags

- `--raw`, `-r`: Path to the RAW root directory.
- `--compressed`, `-c`: Path to the JPEG root directory.
- `--dry`: Dry run (no deletions, prints what would be deleted).
- `--verbose`, `-v`: Print per-file matching and deletion output.
- `--summary-only`: Suppress per-file output, only show summary.

### Examples

Delete orphaned JPEGs (no RAW present):

```bash
target/release/photo-cleanup clean --raw /path/to/raw --compressed /path/to/jpeg
```

Delete JPEGs that already have a RAW file:

```bash
target/release/photo-cleanup clean-matched --raw /path/to/raw --compressed /path/to/jpeg
```

Preview what would be deleted (recommended first step):

```bash
target/release/photo-cleanup clean --raw /path/to/raw --compressed /path/to/jpeg --dry
```

### Safety Notes

- Deletions are permanent. Use `--dry` first to verify the files that would be removed.
- Matching is based on relative path and filename stem only. If you move files between directories, matches may not be detected.

## Development

Run the tool directly with Cargo:

```bash
cargo run -- clean --raw /path/to/raw --compressed /path/to/jpeg --dry
```
