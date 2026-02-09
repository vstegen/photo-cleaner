# Cleaning Up JPEG Files After Deleting RAWs

If you shoot in RAW+JPEG mode, you've probably run into this problem: your external SSD is full, you've spent hours in Lightroom culling your RAW files—deleting the blurry shots, picking the best from a burst of similar frames—but all those corresponding JPEG files are still sitting there, eating up precious disk space.

This is exactly where I found myself recently. I import only RAW files into Lightroom for selection and editing, but my camera dutifully writes both RAW and JPEG to the SD card. After a few trips and photo sessions, my 1TB external SSD was completely full. I'd already done the hard work of curating my RAW files, but the orphaned JPEGs were still around, along with JPEGs for photos I decided to keep.

## The Problem

When you delete RAW files directly (say, in Lightroom or your file browser), the corresponding JPEG files don't go with them. You're left with two scenarios:

1. **Orphaned JPEGs** - JPEG files whose RAW counterparts you've already deleted
2. **Matched JPEGs** - JPEG files that still have their RAW counterparts (maybe you want to keep only the RAWs and reclaim JPEG space)

Manually hunting down and deleting these files across hundreds of folders? No thanks.

## Enter photo-cleanup

I built a simple command-line tool to handle this. It walks through your photo directories and handles both scenarios:

```bash
# Delete JPEGs that don't have a corresponding RAW file
photo-cleanup clean ~/Photos

# Delete JPEGs that DO have a corresponding RAW file
photo-cleanup clean-matched ~/Photos
```

The tool looks for common RAW extensions (CR2, NEF, ARW, DNG, RAF, ORF, RW2) and matches them with their JPEG counterparts based on filename. It supports both JPEG and JPG extensions.

There's also a `--dry-run` flag so you can see what would be deleted before actually pulling the trigger:

```bash
photo-cleanup clean --dry-run ~/Photos
```

## Why Not Just [Insert Photo Manager Here]?

Fair question. Tools like Lightroom, Capture One, and others have their own file management features. But I wanted something simple, fast, and independent of any specific photo management software. Just a CLI tool that does one thing well: matches RAW and JPEG files and cleans up based on simple rules.

It's about 150 lines of Rust, uses basic file system traversal, and gets the job done. Nothing fancy, but it solved my immediate problem and saved me several gigabytes of SSD space.

## Try It Yourself

If you're comfortable with the command line and facing the same storage crunch, give it a shot. The code is on [GitHub](https://github.com/vstegen/photo-cleanup), and you can install it with Cargo:

```bash
cargo install --git https://github.com/vstegen/photo-cleanup
```

Just remember: always use `--dry-run` first. Deleting files is permanent, and while I've tested this on my own photo library, your mileage may vary.

---

*Have a similar workflow or a better solution? I'd love to hear about it.*
