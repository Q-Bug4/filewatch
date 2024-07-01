use std::path::PathBuf;

use crate::processor::Processor;

pub struct Pipeline {
    processors: Vec<Box<dyn Processor>>,
    watch_path: PathBuf,
}

impl Pipeline {
    pub fn new(processors: Vec<Box<dyn Processor>>, watch_path: PathBuf) -> Self {
        Self {
            processors,
            watch_path,
        }
    }
}
