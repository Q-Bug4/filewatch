use std::path::{Path, PathBuf};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use crate::processor::move_file_processor::MoveFileProcessor;
use crate::processor::Processor;

pub struct Pipeline {
    processors: Vec<Box<dyn Processor>>,
    watch_path: PathBuf,
    watcher: RecommendedWatcher
}

impl Pipeline {
    pub fn new(processors: Vec<Box<dyn Processor>>, watch_path: PathBuf) -> Self {
        let watcher = notify::recommended_watcher(|res| {
            match res {
                Ok(event) => {
                    if event.kind.is_create() {
                        for processor in processors {
                            processor.proceed(event.paths[0].clone());
                        }
                    }
                }
                Err(e) => println!("watch error: {:?}", e),
            }
        })?;
        Self {
            processors,
            watch_path,
            watcher
        }
    }

    pub fn stop(&mut self) {
        // stop watcher
        self.watcher.unwatch(Path::new(&self.watch_path)).unwrap();
    }

    pub fn start(&mut self) {
        // use rust notify new version to watch if dir has changed
        self.watcher.watch(Path::new(&self.watch_path), RecursiveMode::Recursive)?;
    }
}
