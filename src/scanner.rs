use std::path::Path;

use walkdir::WalkDir;

use crate::stats::FolderStats;

pub fn scan_folder(folder_path: &str) -> FolderStats {
    let mut stats = FolderStats::new();

    for entry in WalkDir::new(folder_path) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        if entry.file_type().is_file() {
            stats.total_files += 1;

            if let Ok(metadata) = entry.metadata() {
                let file_size = metadata.len();

                stats.total_size += file_size;

                if file_size > stats.largest_file_size {
                    stats.largest_file_size = file_size;
                    stats.largest_file_name = entry.path().display().to_string();
                }
            }

            let extension = Path::new(entry.path())
                .extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("No Extension")
                .to_lowercase();

            *stats.extensions.entry(extension).or_insert(0) += 1;
        } else if entry.file_type().is_dir() {
            stats.total_folders += 1;
        }
    }

    if stats.total_folders > 0 {
        stats.total_folders -= 1;
    }

    stats
}