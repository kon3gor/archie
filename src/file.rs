use chrono::{offset::Local, DateTime};
use std::path::PathBuf;

pub struct FileInfo {
    name: String,
    creation_date: DateTime<Local>,
    modification_date: DateTime<Local>,
    content: String,
}

impl FileInfo {
    pub fn from_file(path: &PathBuf) -> FileInfo {
        let now = Local::now();
        return FileInfo {
            name: "".to_string(),
            creation_date: now,
            modification_date: now,
            content: "".to_string(),
        };
    }
}
