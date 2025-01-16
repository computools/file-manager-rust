use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

use anyhow::Result;

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub formated_name: String,
    pub is_dir: bool,
}

impl File {
    pub fn new(name: String, is_dir: bool) -> Self {
        let formated_name = if is_dir {
            format!("🗀 {name}")
        } else {
            format!("🗎 {name}")
        };

        Self {
            name,
            formated_name,
            is_dir,
        }
    }
}

pub fn list_dir(path: PathBuf, show_hidden: bool) -> Result<Vec<File>> {
    let dir_entries: Vec<DirEntry> = fs::read_dir(path)?.filter_map(|entry| entry.ok()).collect();

    let mut files: Vec<File> = Vec::new();
    for entry in dir_entries {
        let mut file_name = format!("{:?}", entry.file_name());

        file_name = file_name
            .strip_prefix("\"")
            .unwrap_or(&file_name)
            .to_string();

        file_name = file_name
            .strip_suffix("\"")
            .unwrap_or(&file_name)
            .to_string();

        if file_name.starts_with(".") && !show_hidden {
            continue;
        }

        files.push(File::new(file_name, entry.metadata()?.is_dir()));
    }

    files.sort_unstable_by_key(|file| (!file.is_dir, file.name.clone()));

    Ok(files)
}
