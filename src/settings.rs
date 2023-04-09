use config::{Config, ConfigError, File};
use serde::Deserialize;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref conf: Settings = Settings::new().unwrap();
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub debug: bool,
    pub database: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let c = Config::builder()
            .set_default("debug", false)?
            .set_default("database", "postgres://")?
            .add_source(File::with_name("config/default.toml"))
            .build()?;

        c.try_deserialize()
    }
}

