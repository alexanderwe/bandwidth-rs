use failure::Error;
use pretty_bytes::converter::convert;

use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Read};
use std::path::Path;
use std::str;

use config::Config;
use proc;
use std::{env, fs};

const NET_DEV_FILE: &'static str = "/proc/net/dev";

#[derive(Debug)]
struct Interface {
    name: String,
    received_bytes: u64,
    received_packets: u64,
    received_errs: u64,
    received_drop: u64,
    received_fifo: u64,
    received_frame: u64,
    received_compressed: u64,
    received_multicast: u64,
    transmit_bytes: u64,
    transmit_packets: u64,
    transmit_errs: u64,
    transmit_drop: u64,
    transmit_fifo: u64,
    transmit_colls: u64,
    transmit_carrier: u64,
    transmit_compressed: u64,
}

pub fn read_dev(config: &Config) -> Result<String, Error> {
    let mut dir = env::current_exe()?;
    dir.pop();
    dir.push("stats");

    if !Path::new(&dir).exists() {
        fs::create_dir(&dir)?;
    }

    dir.push(&config.interface);

    // println!("{}", stats_filepath);

    if !Path::new(&dir).exists() {
        let mut stats_file = File::create(&dir).map_err(|_| ServiceError::StatsFileCreationError)?;
        stats_file.write_all(b"0\n0").map_err(|_| {
            ServiceError::MissingFileError(
                dir.to_str()
                    .unwrap_or("Error unwrapping file path")
                    .to_string(),
            )
        })?;
    }

    let stats_file = File::open(&dir).map_err(|_| {
        ServiceError::MissingFileError(
            dir.to_str()
                .unwrap_or("Error unwrapping file path")
                .to_string(),
        )
    })?;

    let metadata = fs::metadata(&dir)?;
    let modified_time = metadata.modified()?;

    if modified_time < proc::get_startup_time()? {
        fs::write(&dir, format!("{}\n{}", 0, 0)).map_err(|_| {
            ServiceError::MissingFileError(
                dir.to_str()
                    .unwrap_or("Error unwrapping file path")
                    .to_string(),
            )
        })?;
    }

    let mut stats_vec: Vec<String> = Vec::new();
    let buf_reader = BufReader::new(stats_file);

    for line in buf_reader.lines() {
        let line = line.expect("Unable to read line");
        stats_vec.push(line);
    }

    let mut file = File::open(NET_DEV_FILE)
        .map_err(|_| ServiceError::MissingFileError(String::from(NET_DEV_FILE)))?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|_| ServiceError::MissingFileError(String::from(NET_DEV_FILE)))?;

    let file_content = str::from_utf8(&buffer);
    let split = file_content.unwrap().split("\n");

    let mut vec = split.collect::<Vec<&str>>();
    vec.drain(0..2);

    let mut result: String = String::from("");

    for s in vec {
        if s.len() == 0 {
            continue;
        }
        let vars = s.split_whitespace().collect::<Vec<&str>>();
        let interface = Interface {
            name: vars[0].to_string().replace(":", ""),
            received_bytes: vars[1].parse::<u64>().unwrap(),
            received_packets: vars[2].parse::<u64>().unwrap(),
            received_errs: vars[3].parse::<u64>().unwrap(),
            received_drop: vars[4].parse::<u64>().unwrap(),
            received_fifo: vars[5].parse::<u64>().unwrap(),
            received_frame: vars[6].parse::<u64>().unwrap(),
            received_compressed: vars[7].parse::<u64>().unwrap(),
            received_multicast: vars[8].parse::<u64>().unwrap(),
            transmit_bytes: vars[9].parse::<u64>().unwrap(),
            transmit_packets: vars[10].parse::<u64>().unwrap(),
            transmit_errs: vars[11].parse::<u64>().unwrap(),
            transmit_drop: vars[12].parse::<u64>().unwrap(),
            transmit_fifo: vars[13].parse::<u64>().unwrap(),
            transmit_colls: vars[14].parse::<u64>().unwrap(),
            transmit_carrier: vars[15].parse::<u64>().unwrap(),
            transmit_compressed: vars[16].parse::<u64>().unwrap(),
        };

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
            ).map_err(|_| {
                ServiceError::MissingFileError(
                    dir.to_str()
                        .unwrap_or("Error unwrapping file path")
                        .to_string(),
                )
            })?;
        }
    }

    if result.len() > 0 {
        Ok(result)
    } else {
        Err(Error::from(ServiceError::InterfaceError))
    }
}

#[derive(Debug, Fail)]
pub enum ServiceError {
    #[fail(display = "Failed to listen on interface")]
    InterfaceError,
    #[fail(display = "File {} is missing", _0)]
    MissingFileError(String),
    #[fail(display = "Could not create stats file")]
    StatsFileCreationError,
    #[fail(display = "Could not write to file {}", _0)]
    FileWriteError(String),
}
