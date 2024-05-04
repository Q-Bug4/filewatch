use std::fs;
use std::path::{Path, PathBuf};

pub trait file_event_hanlder {
    // 使用监听器模式，监听器输入文件路径作为参数，handler对文件进行处理，返回处理结果
    fn handle_file_event(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>>;

    // 获取handler的类型，返回一个枚举
    fn get_handler_type(&self) -> HandlerType;
}

// 枚举HandlerType，用于区分不同类型的handler，区分读操作和写操作
pub enum HandlerType {
    Read,
    Write,
}

mod handler {
    use std::fs;
    use std::path::{Path, PathBuf};
    use crate::event_handler::{file_event_hanlder, HandlerType, list_files};

    // 实现file_event_hanlder trait的handler，用于移动文件，类型为Write
    pub struct MoveFileHandler {}

    impl file_event_hanlder for MoveFileHandler {
        fn handle_file_event(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
            // TODO: 实现文件移动逻辑

            Ok(())
        }

        fn get_handler_type(&self) -> HandlerType {
            HandlerType::Write
        }
    }

    // 实现file_event_handler trait的handler，用于重命名文件，类型为Write
    pub struct RenameFileHandler {
        // 添加dump_path字段，用于识别文件在该path下是否有重名
        dump_paths: Vec<String>,
        // 添加是否递归字段，若该字段为true，则递归处理文件夹
        recursive: bool,
    }

    impl RenameFileHandler {
        pub fn new(dump_paths: Vec<String>, recursive: bool) -> Self {
            Self {
                dump_paths,
                recursive,
            }
        }
    }

    impl file_event_hanlder for RenameFileHandler {
        fn handle_file_event(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
            // 从dump_paths中遍历，找到和file_path同名的文件，如果存在，则重命名当前文件，否则直接返回
            for dump_path in &self.dump_paths {
                let path = Path::new(dump_path);
                // 根据recursive的值，判断是否需要递归处理文件夹，如果是，则递归遍历dump_path下文件并写入到一个Vec中。反之则只遍历当前目录
                let files: Vec<PathBuf> = list_files(path, self.recursive)?;

                // 重命名时在文件的拓展名前加入时间戳
                for entry_path in files {
                    if entry_path.file_name().unwrap().to_str().unwrap() == file_path {
                        let new_file_name = format!("{}-{}.{}", file_path, chrono::Local::now().format("%Y%m%d%H%M%S"), entry_path.extension().unwrap().to_str().unwrap());
                        fs::rename(entry_path, new_file_name)?;
                        break;
                    }
               }
            }
            Ok(())
        }


        fn get_handler_type(&self) -> HandlerType {
            HandlerType::Write
        }
    }

}

fn list_files(path: &Path, recursive: bool) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut files = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if recursive {
                files.extend(list_files(&path, recursive)?);
            } else {
                println!("Directory (not recursed into): {:?}", path);
            }
        } else {
            files.push(path);
        }
    }

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use crate::event_handler::handler::RenameFileHandler;

    #[test]
    fn test_handle_file_event() {
        // Prepare test data
        let file_path = "test_file.txt";
        let dump_paths = vec!["./dump".to_string()];
        let recursive = false;
        let handler = RenameFileHandler::new(dump_paths, recursive);

        // Create a test file
        let test_file_path = PathBuf::from(file_path);
        fs::create_dir_all("./dump")?;
        fs::write(&test_file_path, "test data")?;

        // Handle file event
        handler.handle_file_event(file_path).unwrap();

        // Check if the file has been renamed
        let renamed_file_path = format!("{}-{}.txt", file_path, chrono::Local::now().format("%Y%m%d%H%M%S"));
        assert!(fs::metadata(&PathBuf::from(renamed_file_path)).is_ok());

        // Clean up
        fs::remove_file(&test_file_path)?;
        fs::remove_file(&PathBuf::from(renamed_file_path.clone()))?;
        fs::remove_dir_all("./dump")?;
    }

    #[test]
    fn test_list_files() {
        // Prepare test data
        fs::create_dir_all("./test_dir")?;
        fs::write("./test_dir/test_file1.txt", "test data 1")?;
        fs::write("./test_dir/test_file2.txt", "test data 2")?;
        fs::create_dir_all("./test_dir/sub_dir")?;
        fs::write("./test_dir/sub_dir/test_file3.txt", "test data 3")?;

        // List files recursively
        let recursive = true;
        let files = list_files(Path::new("./test_dir"), recursive).unwrap();
        assert_eq!(files.len(), 3);
        assert!(files.contains(&PathBuf::from("./test_dir/test_file1.txt")));
        assert!(files.contains(&PathBuf::from("./test_dir/test_file2.txt")));
        assert!(files.contains(&PathBuf::from("./test_dir/sub_dir/test_file3.txt")));

        // List files non-recursively
        let recursive = false;
        let files = list_files(Path::new("./test_dir"), recursive).unwrap();
        assert_eq!(files.len(), 2);
        assert!(files.contains(&PathBuf::from("./test_dir/test_file1.txt")));
        assert!(files.contains(&PathBuf::from("./test_dir/test_file2.txt")));

        // Clean up
        fs::remove_file("./test_dir/test_file1.txt")?;
        fs::remove_file("./test_dir/test_file2.txt")?;
        fs::remove_file("./test_dir/sub_dir/test_file3.txt")?;
        fs::remove_dir_all("./test_dir")?;
    }
}
