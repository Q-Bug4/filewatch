use std::fs;
use std::path::{Path, PathBuf};
use crate::event_handler::{FileEventHandler, format_filename_with_timestamp, HandlerType};
use crate::event_handler::rename_file_handler::RenameFileHandler;

// 实现file_event_handler trait的handler，用于移动文件，类型为Write
pub struct MoveFileHandler {
    // 添加目标文件夹字段，用于将文件移动到目标文件夹
    target_folder: String,
    // 持有一个RenameFileHandler
    rename_handler: RenameFileHandler,
}

impl MoveFileHandler {
    // 构造函数，初始化目标文件夹和dump_paths字段
    pub fn new(target_folder: String) -> Self {
        Self {
            target_folder: target_folder.clone(),
            rename_handler: RenameFileHandler::new(vec![target_folder.clone()], false),
        }
    }
}

impl FileEventHandler for MoveFileHandler {
    fn handle_file_event(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // 检查文件是否存在，如果不存在则直接返回
        if !Path::new(file_path).exists() {
            return Ok(());
        }

        let mut new_file_path = PathBuf::from(file_path);
        if new_file_path.exists() {
            new_file_path = PathBuf::from(format_filename_with_timestamp(file_path));
        }
        fs::rename(file_path, &new_file_path)?;
        Ok(())
    }

    fn get_handler_type(&self) -> HandlerType {
        HandlerType::Write
    }
}