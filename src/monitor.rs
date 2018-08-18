use failure::{Error, ResultExt};
use pretty_bytes::converter::convert;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::{env, fs};

use config::Config;
use error::ServiceError;
use proc;

pub fn read_dev(config: &Config) -> Result<String, Error> {
    let mut dir = env::current_exe()?;
    dir.pop();
    dir.push("stats");

    if !Path::new(&dir).exists() {
        fs::create_dir(&dir)?;
    }

    dir.push(&config.interface);

    let stats_file_path = dir.to_str()
        .unwrap_or("Stats file path invalid")
        .to_string();

    if !Path::new(&dir).exists() {
        let mut stats_file =
            File::create(&dir).context(ServiceError::FileCreationError(stats_file_path.clone()))?;
        stats_file
            .write_all(b"0\n0")
            .context(ServiceError::FileWriteError(stats_file_path.clone()))?;
    }

    let stats_file =
        File::open(&dir).context(ServiceError::MissingFileError(stats_file_path.clone()))?;

    let metadata = fs::metadata(&dir)?;
    let modified_time = metadata.modified()?;

    if modified_time < proc::get_startup_time()? {
        fs::write(&dir, format!("{}\n{}", 0, 0))
            .context(ServiceError::MissingFileError(stats_file_path.clone()))?;
    }

    let mut stats_vec: Vec<String> = Vec::new();
    let buf_reader = BufReader::new(stats_file);

    for line in buf_reader.lines() {
        let line = line.expect("Unable to read line");
        stats_vec.push(line);
    }

    let mut result: String = String::from("");

    for interface in proc::read_interfaces()? {
        if config.interface == interface.name {
            result = format!(
                "{} {} ⇩{}/s | {} ⇧{}/s ",
                interface.name,
                convert(interface.received_bytes as f64),
                convert((interface.received_bytes - &stats_vec[0].parse::<u64>().unwrap()) as f64),
                convert(interface.transmit_bytes as f64),
                convert((interface.transmit_bytes - &stats_vec[1].parse::<u64>().unwrap()) as f64),
            );
            fs::write(
                &dir,
                format!("{}\n{}", interface.received_bytes, interface.transmit_bytes),
            ).context(ServiceError::MissingFileError(stats_file_path.clone()))?;
        }
    }

    if result.len() > 0 {
        Ok(result)
    } else {
        Err(Error::from(ServiceError::InterfaceError))
    }
}
