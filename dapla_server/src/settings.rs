use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use config::{Config, ConfigError, Environment, File};
use log::Level;
use serde::{de::Error, Deserialize, Deserializer};

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct HttpSettings {
    pub host: String,
    pub port: u16,
    pub web_root: PathBuf,
}

impl Default for HttpSettings {
    fn default() -> Self {
        Self {
            host: "localhost".into(),
            port: 8080,
            web_root: PathBuf::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct LoggerSettings {
    #[serde(deserialize_with = "deserialize_log_level")]
    pub level: Level,
}

fn deserialize_log_level<'de, D>(deserializer: D) -> Result<Level, D::Error>
where
    D: Deserializer<'de>,
{
    let level = String::deserialize(deserializer)?;
    Level::from_str(&level).map_err(Error::custom)
}

impl Default for LoggerSettings {
    fn default() -> Self {
        Self { level: Level::Debug }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct DapsSettings {
    pub path: PathBuf,
}

impl Default for DapsSettings {
    fn default() -> Self {
        Self { path: "daps".into() }
    }
}

#[derive(Default, Debug, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub http: HttpSettings,
    pub log: LoggerSettings,
    pub daps: DapsSettings,
}

impl Settings {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let mut settings = Config::new();
        settings
            .merge(File::from(path.as_ref()))?
            // Add in settings from the environment (with a prefix of DAPLA)
            // Eg.. `DAPLA_HTTP.PORT=8090 dapla_server` would set the `http.port` key
            .merge(Environment::with_prefix("DAPLA").separator("."))?;
        settings.try_into()
    }
}
