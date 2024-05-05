use std::fs;
use std::path::Path;
use crate::event_handler::{FileEventHandler, HandlerType};

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
        // 从dump_paths中遍历，找到和file_path同名的文件，如果存在，则重命名file_path，否则直接返回
        !todo!()
    }


    fn get_handler_type(&self) -> HandlerType {
        HandlerType::Write
    }
}

impl RenameFileHandler {
    // 在dump_paths中遍历所有文件，找到与file_path同名的文件，方法名为should_rename(file_path: String) -> bool
    fn should_rename(&self, file_path: String) -> bool {
        for path in &self.dump_paths {
            if Path::new(path).exists() {
                for entry in fs::read_dir(path).unwrap() {
                    let entry = entry.unwrap();
                    let entry_path = entry.path();
                    if entry_path.file_name().unwrap().to_str().unwrap() == file_path {
                        return true;
                    }
                }
            }
        }
        false
    }

}