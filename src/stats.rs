use std::collections::HashMap;

pub struct FolderStats {
    pub total_files: usize,
    pub total_folders: usize,
    pub total_size: u64,

    pub largest_file_name: String,
    pub largest_file_size: u64,

    pub extensions: HashMap<String, usize>,
}

impl FolderStats {
    pub fn new() -> Self {
        Self {
            total_files: 0,
            total_folders: 0,
            total_size: 0,

            largest_file_name: String::new(),
            largest_file_size: 0,

            extensions: HashMap::new(),
        }
    }
}