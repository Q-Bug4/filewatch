use crate::event_handler::Processor;

pub struct Pipeline {
    handlers: Vec<Box<dyn Processor>>,
}

impl Pipeline {
    pub fn new(handlers: Vec<Box<dyn Processor>>) -> Self {
        Self {
            handlers,
        }
    }

    pub fn run(&self) {
        for handler in &self.handlers {
            handler.process();
        }
    }
}
