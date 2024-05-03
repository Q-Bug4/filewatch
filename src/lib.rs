#[derive(Debug, Clone)]
pub struct InputConfig {
    path: String,
}

impl InputConfig {
    fn new(path: String) -> Self {
        Self { path }
    }
}

pub fn get_input_config(args: Vec<String>) -> Result<InputConfig, String> {
    match args.len() {
        0 | 1 => Err("Args is empty!".to_string()),
        _ => Ok(InputConfig::new(args[1].clone())),
    }
}
