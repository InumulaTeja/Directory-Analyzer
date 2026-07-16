mod report;
mod scanner;
mod stats;
mod utils;

use std::env;

use report::print_report;
use scanner::scan_folder;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: cargo run -- <folder_path>");
        return;
    }

    let folder_path = &args[1];

    let stats = scan_folder(folder_path);

    print_report(folder_path, &stats);
}