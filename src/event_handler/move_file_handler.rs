use std::fs;
use std::path::PathBuf;

use crate::event_handler::{format_filename_with_timestamp, is_file_exist, Processor};

// 实现file_event_handler trait的handler，用于移动文件，类型为Write
pub struct MoveFileHandler {
    // 添加目标文件夹字段，用于将文件移动到目标文件夹
    target_folder: PathBuf,
}

impl MoveFileHandler {
    // 构造函数，初始化目标文件夹和dup_paths字段
    pub fn new(target_folder: PathBuf) -> Self {
        Self {
            target_folder: target_folder.clone(),
        }
    }
}

impl Processor for MoveFileHandler {
    fn proceed(&self, file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        // 检查文件是否存在，如果不存在则直接返回
        if !file_path.exists() {
            eprintln!("{} not exist", file_path.to_str().unwrap());
            return Ok(());
        }

        let mut new_file_path = PathBuf::from(file_path);
        let has_dup = is_file_exist(&self.target_folder, file_path.file_name().unwrap().to_str().unwrap().to_string());
        if has_dup {
            new_file_path = PathBuf::from(file_path).with_file_name(format_filename_with_timestamp(file_path.to_str().unwrap()));
            fs::rename(file_path, &new_file_path)?;
        }

        Ok(())
    }

    fn get_name() -> String {
        "move".to_string()
    }
}

