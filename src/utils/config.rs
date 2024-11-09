use dotenv::dotenv;
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

#[derive(Clone, Debug)]
pub struct Config {
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
    pub aws_region: String,
    pub aws_id: String,
    pub log_level: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        Self {
            aws_access_key_id: get_required_var("AWS_ACCESS_KEY_ID"),
            aws_secret_access_key: get_required_var("AWS_SECRET_ACCESS_KEY"),
            aws_region: get_var_with_default("AWS_REGION", "us-east-1"),
            aws_id: get_required_var("AWS_ID"),
            log_level: get_var_with_default("RUST_LOG", "info"),
        }
    }

    pub fn init_logging(&self) {
        env_logger::init();
    }
}

fn get_required_var(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("{} must be set", key))
}

fn get_var_with_default(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| String::from(default))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_config_loads_defaults() {
        // Clear any existing env vars
        env::remove_var("AWS_REGION");
        env::remove_var("RUST_LOG");

        let config = Config::new();
        assert_eq!(config.aws_region, "us-east-1");
        assert_eq!(config.log_level, "info");
    }

    #[test]
    fn test_config_loads_custom_values() {
        env::set_var("AWS_REGION", "eu-west-1");
        env::set_var("RUST_LOG", "debug");

        let config = Config::new();
        assert_eq!(config.aws_region, "eu-west-1");
        assert_eq!(config.log_level, "debug");
    }
}
