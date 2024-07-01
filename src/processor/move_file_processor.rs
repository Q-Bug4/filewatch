use std::fs;
use std::path::PathBuf;

use crate::processor::{format_filename_with_timestamp, is_file_exist, Processor};

// 实现file_event_processor trait的processor，用于移动文件，类型为Write
pub struct MoveFileProcessor {
    // 添加目标文件夹字段，用于将文件移动到目标文件夹
    target_folder: PathBuf,
}

impl MoveFileProcessor {
    // 构造函数，初始化目标文件夹和dup_paths字段
    pub fn new(target_folder: PathBuf) -> Self {
        Self {
            target_folder: target_folder.clone(),
        }
    }
}

impl Processor for MoveFileProcessor {
    /**
        move file into target_folder.
        - when file is not exist, return error.
        - when file exist in target_folder, rename file with timestamp.
        - or just move file into target_folder
    */
    fn proceed(&self, file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        // 检查文件是否存在，如果不存在则直接返回
        if !file_path.exists() {
            // 返回错误信息
            return Err(format!("{} not exist", file_path.to_str().unwrap()).into());
        }

        let has_dup = is_file_exist(&self.target_folder, file_path.file_name().unwrap().to_str().unwrap().to_string());
        let new_file_name = if has_dup {
            format_filename_with_timestamp(file_path.file_name().unwrap().to_str().unwrap())
        } else {
            file_path.file_name().unwrap().to_str().unwrap().to_string()
        };
        // 以target_folder/new_file_name生成目标路径
        let new_file_path = self.target_folder.join(new_file_name);
        fs::rename(file_path, &new_file_path)?;

        Ok(())
    }


    fn get_name(&self) -> String {
        "move".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::tempdir;

    fn create_temp_file(dir: &PathBuf, filename: &str, content: &str) -> PathBuf {
        let file_path = dir.join(filename);
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "{}", content).unwrap();
        file_path
    }

    fn format_filename_with_timestamp(filename: &str) -> String {
        format!("{}_timestamp", filename)
    }

    fn is_file_exist(target_folder: &PathBuf, filename: String) -> bool {
        target_folder.join(filename).exists()
    }

    #[test]
    fn test_proceed_file_exists_no_dup() {
        let temp_dir = tempdir().unwrap();
        let target_dir = temp_dir.path().join("target");
        fs::create_dir(&target_dir).unwrap();

        let file_path = create_temp_file(&temp_dir.path().to_path_buf(), "test.txt", "test content");
        let processor = MoveFileProcessor::new(target_dir.clone());

        let result = processor.proceed(&file_path);
        assert!(result.is_ok());

        let moved_file_path = target_dir.join("test.txt");
        assert!(moved_file_path.exists());
    }

    #[test]
    fn test_proceed_file_exists_with_dup() {
        let temp_dir = tempdir().unwrap();
        let target_dir = temp_dir.path().join("target");
        fs::create_dir(&target_dir).unwrap();

        let file_path = create_temp_file(&temp_dir.path().to_path_buf(), "test.txt", "test content");
        create_temp_file(&target_dir, "test.txt", "existing content");

        let processor = MoveFileProcessor::new(target_dir.clone());

        let result = processor.proceed(&file_path);
        assert!(result.is_ok());
        assert!(!file_path.exists());
    }

    #[test]
    fn test_proceed_file_does_not_exist() {
        let temp_dir = tempdir().unwrap();
        let target_dir = temp_dir.path().join("target");
        fs::create_dir(&target_dir).unwrap();

        let file_path = temp_dir.path().join("nonexistent.txt");
        let processor = MoveFileProcessor::new(target_dir.clone());

        let result = processor.proceed(&file_path);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            format!("{} not exist", file_path.to_str().unwrap())
        );
    }
}
