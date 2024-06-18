use std::fs;
use std::path::{Path, PathBuf};

mod move_file_handler;
pub mod rename_file_handler;

pub trait Processor {
    // 使用监听器模式，监听器输入文件路径作为参数，handler对文件进行处理，返回处理结果
    fn proceed(&self, file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>>;

    fn get_name() -> String;
}

pub struct ProcessorFailure {}

fn is_file_exist(target_path: &PathBuf, file_name: String) -> bool {
    // 判断target_path下是否有文件file_name
    let file_path = target_path.join(file_name);
    file_path.exists()
}

pub fn format_filename_with_timestamp(filename: &str) -> String {
    let now = chrono::Local::now();
    let timestamp = now.format("%Y%m%d%H%M%S").to_string();
    let extension = Path::new(filename).extension().unwrap().to_str().unwrap();
    let src_filename = Path::new(filename).file_stem().unwrap().to_str().unwrap();
    format!("{}-{}.{}", src_filename, timestamp, extension)
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
