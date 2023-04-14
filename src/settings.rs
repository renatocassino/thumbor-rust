use std::sync::{Arc, RwLock};

use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
#[allow(unused)]
pub struct Settings {
    pub debug: bool,
    pub secret_key: String,
}

impl Settings {
    pub fn from_file() -> Result<Self, ConfigError> {
        let c = Config::builder()
            .set_default("debug", false)?
            .set_default("secret_key", "")?
            .add_source(File::with_name("config/default.toml"))
            .build()?;

        c.try_deserialize()
    }

    pub fn current() -> Arc<Settings> {
        CONF.with(|c| c.read().unwrap().clone())
    }

    pub fn make_current(self) {
        CONF.with(|c| *c.write().unwrap() = Arc::new(self))
    }

    pub fn start() {
        let config = Settings::from_file().unwrap();
        CONF.with(|c| *c.write().unwrap() = Arc::new(config));
    }
}

thread_local! {
    static CONF: RwLock<Arc<Settings>> = RwLock::new(Default::default());
}

pub fn conf() -> Arc<Settings> {
    Settings::current()
}