extern crate toml;

use failure::Error;
use std::fs::File;
use std::{env, fs};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub interface: String,
}

pub fn get_config() -> Result<Config, Error> {
    let mut dir = env::current_exe()?;
    dir.pop();
    dir.push("config.toml");

    let f = File::open(&dir).map_err(|_| ConfigError::MissingConfigFile)?;

    let content = fs::read_to_string(&dir).map_err(|_| ConfigError::CannotReadConfigFile)?;

    let decoded: Config = toml::from_str(&content).map_err(|_| ConfigError::InvalidConfigFile)?;

    Ok(decoded)
}

#[derive(Debug, Fail)]
pub enum ConfigError {
    #[fail(display = "Could not find config.toml")]
    MissingConfigFile,
    #[fail(display = "Could not read config.toml")]
    CannotReadConfigFile,
    #[fail(display = "Invalid config.toml")]
    InvalidConfigFile,
}
