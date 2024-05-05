mod move_file_handler;
mod rename_file_handler;

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

fn format_filename_with_timestamp(file_path: &str) -> String {
    let now = chrono::Local::now();
    let timestamp = now.format("%Y%m%d%H%M%S").to_string();
    let extension = Path::new(file_path).extension().unwrap().to_str().unwrap();
    format!("{}-{}.{}", file_path, timestamp, extension)
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
