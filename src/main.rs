use clap::Parser;
use rayon::iter::{ParallelBridge, ParallelIterator};
use sha2::Digest;
use std::{
    collections::HashSet,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

/// Simple CLI to list files in `right` that are not in `left` (based on sha256 hash)
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// The "left" folder
    left: PathBuf,
    /// The "right" folder
    right: PathBuf,
    /// Print progress as files are hashed
    #[arg(short, long)]
    verbose: bool,
    /// Only report the first file per hash in `right`, ignore duplicates
    #[arg(short = 'i', long = "ignore-duplicates")]
    ignore_duplicates: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Hash all files in left (in parallel)
    let left_entries = hash_all_files(&args.left)?;
    let left_hashes: HashSet<String> = left_entries.iter().map(|(h, _)| h.clone()).collect();

    let mut right_entries = hash_all_files(&args.right)?;

    // Optionally remove duplicates from 'right'
    if args.ignore_duplicates {
        right_entries = deduplicate_by_hash(right_entries);
    }

    println!("Files in {:?} but not in {:?}:", &args.right, &args.left);

    for (hash, path) in right_entries {
        if !left_hashes.contains(&hash) {
            println!("{hash} {}", path.display());
        }
    }

    Ok(())
}

fn hash_all_files(dir: &Path) -> anyhow::Result<Vec<(String, PathBuf)>> {
    let entries = WalkDir::new(dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .par_bridge()
        .filter_map(|entry: walkdir::DirEntry| {
            let path = entry.path().to_path_buf();
            let hash = match compute_sha256(&path) {
                Ok(hash) => hash,
                Err(err) => {
                    eprintln!("Error hashing {:?}: {}", path, err);
                    return None;
                }
            };
            Some((hash, path))
        })
        .collect::<Vec<(String, PathBuf)>>();

    Ok(entries)
}

fn compute_sha256(path: &Path) -> anyhow::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = sha2::Sha256::new();
    let mut buffer = [0u8; 8192];

    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(format!("{:x}", hasher.finalize()))
}

/// If `--ignore-duplicates` is set, we only keep the first occurrence of each hash.
fn deduplicate_by_hash(entries: Vec<(String, PathBuf)>) -> Vec<(String, PathBuf)> {
    let mut seen = HashSet::new();
    let mut deduped = Vec::new();
    for (hash, path) in entries.into_iter() {
        if !seen.contains(&hash) {
            seen.insert(hash.clone());
            deduped.push((hash, path));
        }
    }
    deduped
}
