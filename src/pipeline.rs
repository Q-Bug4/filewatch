use crate::processor::Processor;

pub struct Pipeline {
    processors: Vec<Box<dyn Processor>>,
}

impl Pipeline {
    pub fn new(processors: Vec<Box<dyn Processor>>) -> Self {
        Self {
            processors,
        }
    }

    pub fn run(&self) {
        for processor in &self.processors {
            processor.process();
        }
    }
}
