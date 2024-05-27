use std::fs;
use std::path::PathBuf;

use crate::event_handler::{Processor, format_filename_with_timestamp, ProcessorFailure};
use crate::event_handler::rename_file_handler::RenameFileHandler;

// 实现file_event_handler trait的handler，用于移动文件，类型为Write
pub struct MoveFileHandler {
    // 添加目标文件夹字段，用于将文件移动到目标文件夹
    target_folder: PathBuf,
    // 持有一个RenameFileHandler
    rename_handler: RenameFileHandler,
}

impl MoveFileHandler {
    // 构造函数，初始化目标文件夹和dup_paths字段
    pub fn new(target_folder: PathBuf) -> Self {
        Self {
            target_folder: target_folder.clone(),
            rename_handler: RenameFileHandler::new(vec![target_folder], false),
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
        let has_dup = self.rename_handler.should_rename(file_path.file_name().unwrap().to_str().unwrap().to_string(), &self.target_folder);
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

#[cfg(test)]
mod tests {
    use super::*;

    const target_folder: &'static str = "./tmp/target";

    #[test]
    fn test_handle_file_event_file_exists() {
        // Arrange
        let target_path = PathBuf::from(target_folder);
        let handler = MoveFileHandler::new(target_path.clone());
        let file_path = PathBuf::from("./tmp/file.txt");

        // Act
        fs::create_dir_all(&target_path).unwrap();
        fs::write(&file_path, "test data").unwrap();
        handler.proceed(&file_path).unwrap();

        // Assert
        assert!(!file_path.exists());
        let new_file_path = target_path.join("file.txt");
        assert!(new_file_path.exists());
        let contents = fs::read_to_string(&new_file_path).unwrap();
        assert_eq!("test data", contents);
    }

    #[test]
    fn test_handle_file_event_file_not_exists() {
        // Arrange
        let handler = MoveFileHandler::new(PathBuf::from(target_folder));
        let file_path = PathBuf::from("./tmp/file2.txt");

        // Act
        handler.proceed(&file_path).unwrap();

        // Assert
        assert!(!file_path.exists());
    }
}
