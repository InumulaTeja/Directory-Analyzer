use crate::stats::FolderStats;
use crate::utils::format_size;

pub fn print_report(folder_path: &str, stats: &FolderStats) {
    println!("==================================================");
    println!("              DIRECTORY ANALYZER");
    println!("==================================================");

    println!("Folder        : {}", folder_path);
    println!("Files         : {}", stats.total_files);
    println!("Folders       : {}", stats.total_folders);
    println!("Total Size    : {}", format_size(stats.total_size));
    println!("Largest File  : {}", stats.largest_file_name);
    println!(
        "Largest Size  : {}",
        format_size(stats.largest_file_size)
    );
}