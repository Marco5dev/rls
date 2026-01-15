use std::fs;
use std::path::PathBuf;
use crate::lib::args::Args;
use super::file_entry::FileEntry;

pub fn collect_entries(path: &PathBuf, args: &Args) -> Vec<FileEntry> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if !args.all && name.starts_with('.') {
                continue;
            }
            if let Ok(metadata) = entry.metadata() {
                files.push(FileEntry {
                    name,
                    path: entry.path(),
                    metadata,
                });
            }
        }
    }
    // Sorting
    match args.sort.as_str() {
        "size" => files.sort_by_key(|e| e.metadata.len()),
        "time" => {
            #[cfg(unix)]
            files.sort_by_key(|e| {
                use std::os::unix::fs::MetadataExt;
                -(e.metadata.mtime())
            });
        }
        _ => files.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase())),
    }
    if args.reverse {
        files.reverse();
    }
    files
}
