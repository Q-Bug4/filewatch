use std::path::PathBuf;

use notify::{Event, RecursiveMode, Watcher};

use crate::processor::move_file_processor::MoveFileProcessor;
use crate::processor::Processor;

mod processor;
mod pipeline;

fn main() -> Result<(), String> {
    let path = "/home/qbug/tmp";
    let processor = MoveFileProcessor::new(PathBuf::from(path).join("target"));
    let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
        match res{
            Ok(event) => {
                if event.kind.is_create() {
                    let _ = processor.proceed(&event.paths[0].clone());
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }).unwrap();
    watcher.watch(path.as_ref(), RecursiveMode::Recursive).unwrap();
    loop {
        println!("sleep...");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
