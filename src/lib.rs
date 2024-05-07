mod event_handler;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_input_config_success() {
        let args = vec!["input_config.rs".to_string(), "test".to_string()];
        let result = get_input_config(args);
        assert_eq!(Ok(InputConfig { path: "test".to_string() }), result);
    }

    #[test]
    fn test_get_input_config_no_args() {
        let args = vec![];
        let result = get_input_config(args);
        assert_eq!(Err("Args is empty!".to_string()), result);
    }

    #[test]
    fn test_get_input_config_only_one_arg() {
        let args = vec!["input_config.rs".to_string()];
        let result = get_input_config(args);
        assert_eq!(Err("Args is empty!".to_string()), result);
    }

    #[test]
    fn test_get_input_config_two_args() {
        let args = vec!["input_config.rs".to_string(), "test".to_string(), "args".to_string()];
        let result = get_input_config(args);
        assert_eq!(Ok(InputConfig { path: "test".to_string() }), result);
    }
}
