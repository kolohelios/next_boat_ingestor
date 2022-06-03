use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub redis: deadpool_redis::Config,
    pub wsf_api_key: String
}

impl Config {
    pub fn from_env() -> Result<Self, ::config_crate::ConfigError> {
        let mut cfg = ::config_crate::Config::new();
        cfg.merge(::config_crate::Environment::new().separator("__"))?;
        cfg.try_into()
    }
}
