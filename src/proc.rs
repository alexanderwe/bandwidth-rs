use failure::Error;

use std::fs::File;
use std::io::prelude::*;
use std::str;
use std::time::{Duration, SystemTime};

use monitor::ServiceError;

const UPTIME_FILE: &'static str = "/proc/uptime";

pub fn get_startup_time() -> Result<SystemTime, Error> {
    let mut file = File::open(UPTIME_FILE)
        .map_err(|_| ServiceError::MissingFileError(String::from(UPTIME_FILE)))?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|_| ServiceError::MissingFileError(String::from(UPTIME_FILE)))?;

    let file_content = str::from_utf8(&buffer);

    let uptime_sec = file_content
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()[0]
        .parse::<f64>()
        .unwrap();

    Ok(SystemTime::now() - Duration::from_secs(uptime_sec as u64))
}
