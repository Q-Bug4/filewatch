use std::path::{Path, PathBuf};
use notify::{RecursiveMode, Watcher};
use crate::pipeline::Pipeline;
use crate::processor::move_file_processor::MoveFileProcessor;

mod processor;
mod pipeline;

fn main() -> Result<(), String> {
    let processor = MoveFileProcessor::new(PathBuf::from("/home/qbug/tmp/target"));
    let mut pipeline = Pipeline::new(vec![Box::new(processor)], PathBuf::from("/home/qbug/tmp"));
    pipeline.start();
    loop {
        println!("sleep...");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    Ok(())
}
