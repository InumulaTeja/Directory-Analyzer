mod scanner;
mod stats;
mod utils;

use std::env;

use scanner::scan_folder;
use utils::format_size;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run -- <folder_path>");
        return;
    }

    let folder_path = &args[1];

    let stats = scan_folder(folder_path);

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