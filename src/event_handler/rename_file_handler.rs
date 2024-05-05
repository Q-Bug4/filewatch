use std::fs;
use std::path::{Path, PathBuf};

use crate::event_handler::{FileEventHandler, format_filename_with_timestamp, HandlerType};

// 实现file_event_handler trait的handler，用于重命名文件，类型为Write
pub struct RenameFileHandler {
    // 添加dup_path字段，用于识别文件在该path下是否有重名
    dup_paths: Vec<PathBuf>,
    // 添加是否递归字段，若该字段为true，则递归处理文件夹
    recursive: bool,
}

impl RenameFileHandler {
    pub fn new(dup_paths: Vec<PathBuf>, recursive: bool) -> Self {
        Self {
            dup_paths,
            recursive,
        }
    }
}

impl FileEventHandler for RenameFileHandler {
    fn handle_file_event(&self, file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let filename = Path::new(file_path).file_name().unwrap().to_str().unwrap();

        let result = self.dup_paths.iter().find(|&path| self.should_rename(filename.to_string(), path));
        if result.is_some() {
            let new_filename = format_filename_with_timestamp(file_path.file_name().unwrap().to_str().unwrap());
            let new_file_path = PathBuf::from(file_path).with_file_name(new_filename);
            fs::rename(file_path, new_file_path)?;
        }

        Ok(())
    }

    fn get_handler_type(&self) -> HandlerType {
        HandlerType::Write
    }
}

impl RenameFileHandler {
    pub fn should_rename(&self, filename: String, target_path: &PathBuf) -> bool {
        if target_path.exists() {
            for entry in fs::read_dir(target_path).unwrap() {
                let entry = entry.unwrap();
                let entry_path = entry.path();
                if entry_path.file_name().unwrap().to_str().unwrap() == filename {
                    return true;
                }
            }
        }
        false
    }
}