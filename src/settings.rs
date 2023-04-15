use std::sync::{Arc, RwLock};

use config::{Config, ConfigError, File};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::env;

lazy_static! {
    pub static ref CONF_S: Settings = Settings::from_file().unwrap();
}

#[derive(Debug, Deserialize, Default, Clone)]
#[allow(unused)]
pub struct Settings {
    pub debug: bool,
    pub secret_key: String,
}

impl Settings {
    pub fn from_file() -> Result<Self, ConfigError> {
        let file_config_path = env::var("CONFIG_PATH").unwrap_or("".to_string());

        let c = Config::builder()
            .set_default("debug", false)?
            .set_default("secret_key", "")?
            .add_source(File::with_name("config/default.toml"));

        if file_config_path != "" {
            let c = c.add_source(File::with_name(&file_config_path));
            return c.build()?.try_deserialize();
        }

        c.build()?.try_deserialize()
    }

    pub fn current() -> Arc<Settings> {
        CONF.with(|c| c.read().unwrap().clone())
    }

    pub fn make_current(self) {
        CONF.with(|c| *c.write().unwrap() = Arc::new(self))
    }

    pub fn start() {
        let current_conf: Settings = CONF_S.clone();
        CONF.with(|c| *c.write().unwrap() = Arc::new(current_conf));
    }
}

thread_local! {
    static CONF: RwLock<Arc<Settings>> = RwLock::new(Default::default());
}

pub fn conf() -> Arc<Settings> {
    Settings::current()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings() {
        Settings::start();
        let conf = Settings::current();
        assert_eq!(conf.debug, false);
        assert_eq!(conf.secret_key, "");
    }

    #[test]
    fn test_settings_from_file() {
        let conf = Settings::current();

        Settings {
            secret_key: "ANY_KEY".to_string(),
            debug: true,
            ..Default::default()
        }.make_current();
        let new_conf = Settings::current();

        assert_eq!(conf.debug, false);
        assert_eq!(conf.secret_key, "");

        assert_eq!(new_conf.debug, true);
        assert_eq!(new_conf.secret_key, "ANY_KEY");
    }
}