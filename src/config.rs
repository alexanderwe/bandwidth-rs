extern crate toml;

use clap;
use failure::Error;
use std::path::PathBuf;
use std::{env, fs};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub interface: String,
    pub looping: bool,
}

pub fn get_config(matches: &clap::ArgMatches) -> Result<Config, Error> {
    let mut dir: PathBuf;

    if matches.is_present("config") {
        dir = PathBuf::from(matches.value_of("config").unwrap());
    } else {
        dir = env::current_exe()?;
        dir.pop();
        dir.push("config.toml");
    }

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
