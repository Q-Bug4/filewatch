use std::fs;
use std::path::{Path, PathBuf};

pub trait FileEventHandler {
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
    use crate::event_handler::{FileEventHandler, HandlerType, list_files};

    // 实现file_event_hanlder trait的handler，用于移动文件，类型为Write
    pub struct MoveFileHandler {}

    impl FileEventHandler for MoveFileHandler {
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

    impl FileEventHandler for RenameFileHandler {
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
