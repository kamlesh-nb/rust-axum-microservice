use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct Database {
    pub key: String,
    pub account: String,
    pub db: String,
    pub container: String,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct AppInsight {
  pub key: String,
}

#[derive(Debug, Clone, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub debug: bool,
    pub appinsight: AppInsight,
    pub database: Database,
}

impl Settings {
    pub fn build() -> Result<Self, ConfigError> {
        let axum_mode = env::var("AXUM_ENV").unwrap_or_else(|_| "Dev".into());
        let path = String::from(env::current_dir().unwrap().to_str().unwrap());
        let s = Config::builder()
            .add_source(File::with_name(format!("{}/appconfig.{}.toml", path, axum_mode).as_str()))
            .build()?;

        s.try_deserialize()
    }
}

