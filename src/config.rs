extern crate toml;

use clap;
use failure::{Error, ResultExt};

use std::fs::File;
use std::path::PathBuf;
use std::{env, fs};

use error::ConfigError;

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

    let mut _file = File::open(&dir).context("Config file is missing")?;
    let content = fs::read_to_string(&dir).context(ConfigError::InvalidConfigFile)?;

    let decoded: Config = toml::from_str(&content).context(ConfigError::InvalidConfigFile)?;

    Ok(decoded)
}
