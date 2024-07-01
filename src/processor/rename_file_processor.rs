use std::fs;
use std::path::{Path, PathBuf};

use crate::processor::{Processor, format_filename_with_timestamp, list_files};

// 实现file_event_processor trait的processor，用于重命名文件，类型为Write
pub struct RenameFileProcessor {
    // 添加dup_path字段，用于识别文件在该path下是否有重名
    dup_paths: Vec<PathBuf>,
    recursive: bool,
}

impl RenameFileProcessor {
    pub fn new(dup_paths: Vec<PathBuf>, recursive: bool) -> Self {
        Self {
            dup_paths,
            recursive,
        }
    }
}

impl Processor for RenameFileProcessor {
    fn proceed(&self, file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let filename = Path::new(file_path).file_name().unwrap().to_str().unwrap();

        let result = self.dup_paths.iter()
            .filter(|&path| path != file_path.parent().unwrap())
            .find(|&path| self.should_rename(filename.to_string(), path));
        if result.is_some() {
            let new_filename = format_filename_with_timestamp(file_path.file_name().unwrap().to_str().unwrap());
            let new_file_path = PathBuf::from(file_path).with_file_name(new_filename);
            fs::rename(file_path, new_file_path)?;
        }

        Ok(())
    }

    fn get_name(&self) -> String {
        "rename".to_string()
    }
}

impl RenameFileProcessor {
    pub fn should_rename(&self, filename: String, target_path: &PathBuf) -> bool {
        if target_path.exists() {
            // 调用list_files方法，判断filename在不在该目录下的文件里
            let files: Result<Vec<PathBuf>, std::io::Error> = list_files(target_path, self.recursive);
            if files.is_ok() {
                for file in files.unwrap() {
                    if file.file_name().unwrap().to_str().unwrap() == filename {
                        return true;
                    }
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;
    use tempfile::Builder;

    #[test]
    fn test_handle_file_event_no_duplicate() {
        // 准备工作：创建临时目录和文件
        let tmp_dir = Builder::new().prefix("test_no_duplicate").tempdir().unwrap();
        let file_path = tmp_dir.path().join("test.txt");
        File::create(&file_path).unwrap().write_all(b"content").unwrap();

        // 初始化重命名处理器
        let processor = RenameFileProcessor::new(vec![tmp_dir.path().to_path_buf()], true);

        // 执行处理事件
        processor.proceed(&file_path.clone().into()).unwrap();

        // 验证：原文件不应被重命名
        assert!(&file_path.exists());

        // 清理
        fs::remove_dir_all(tmp_dir.path()).unwrap();
    }

    #[test]
    fn test_handle_file_event_with_duplicate() {
        // 创建两个临时目录，一个存放原始文件，一个模拟已存在同名文件的目录
        let tmp_dir_original = Builder::new().prefix("test_original").tempdir().unwrap();
        let file_path = tmp_dir_original.path().join("duplicate.txt");
        File::create(&file_path).unwrap().write_all(b"content").unwrap();

        let tmp_dir_duplicate = Builder::new().prefix("test_duplicate").tempdir().unwrap();
        File::create(tmp_dir_duplicate.path().join("duplicate.txt")).unwrap(); // 创建同名文件

        // 初始化重命名处理器，指向可能含有重复文件的目录
        let processor = RenameFileProcessor::new(vec![tmp_dir_duplicate.path().to_path_buf()], true);

        // 执行处理事件
        processor.proceed(&file_path.clone().into()).unwrap();

        // 验证：原文件应被重命名
        assert!(!file_path.exists()); // 假设重命名成功

        // 清理
        fs::remove_dir_all(tmp_dir_original.path()).unwrap();
        fs::remove_dir_all(tmp_dir_duplicate.path()).unwrap();
    }

    // 测试handle_file_event，当file_path不存在时的场景
    #[test]
    fn test_handle_file_event_file_not_exists() {
        // 准备工作：创建临时目录和文件
        let tmp_dir = Builder::new().prefix("test_no_duplicate").tempdir().unwrap();
        let file_path = tmp_dir.path().join("test.txt");

        // 初始化重命名处理器
        let processor = RenameFileProcessor::new(vec![tmp_dir.path().to_path_buf()], true);

        // 执行处理事件
        let result = processor.proceed(&file_path);
        assert!(result.is_ok());

        // 清理目录和文件
        fs::remove_dir_all(tmp_dir.path()).unwrap();
   }
}
