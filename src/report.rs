use crate::stats::FolderStats;
use crate::utils::format_size;

pub fn print_report(folder_path: &str, stats: &FolderStats) {
    println!("==================================================");
    println!("              DIRECTORY ANALYZER");
    println!("==================================================");

    println!("Folder                  : {}", folder_path);
    println!("Files                   : {}", stats.total_files);
    println!("Folders                 : {}", stats.total_folders);
    println!("Files Without Extension : {}", stats.files_without_extension);
    println!("Total Size              : {}", format_size(stats.total_size));

    if stats.total_files > 0 {
        println!(
            "Average File Size       : {}",
            format_size(stats.total_size / stats.total_files as u64)
        );
    }

    println!("Largest File            : {}", stats.largest_file_name);
    println!(
        "Largest File Size       : {}",
        format_size(stats.largest_file_size)
    );

    println!();
    println!("Top File Extensions");
    println!("------------------------------");

    let mut extensions: Vec<_> = stats.extensions.iter().collect();

    extensions.sort_by(|a, b| b.1.cmp(a.1));

    for (extension, count) in extensions.iter().take(5) {
        println!("{:<12} {}", extension, count);
    }

    println!();
    println!("Top 5 Largest Files");
    println!("------------------------------");

    for (i, (file, size)) in stats.largest_files.iter().enumerate() {
        println!("{}. {} ({})", i + 1, file, format_size(*size));
    }
}