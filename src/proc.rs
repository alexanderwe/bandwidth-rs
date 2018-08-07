use failure::Error;

use std::fs::File;
use std::io::prelude::*;
use std::str;
use std::time::{Duration, SystemTime};

use monitor::ServiceError;

const UPTIME_FILE: &'static str = "/proc/uptime";
const NET_DEV_FILE: &'static str = "/proc/net/dev";

#[derive(Debug)]
pub struct Interface {
    pub name: String,
    pub received_bytes: u64,
    pub received_packets: u64,
    pub received_errs: u64,
    pub received_drop: u64,
    pub received_fifo: u64,
    pub received_frame: u64,
    pub received_compressed: u64,
    pub received_multicast: u64,
    pub transmit_bytes: u64,
    pub transmit_packets: u64,
    pub transmit_errs: u64,
    pub transmit_drop: u64,
    pub transmit_fifo: u64,
    pub transmit_colls: u64,
    pub transmit_carrier: u64,
    pub transmit_compressed: u64,
}

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

pub fn read_interfaces() -> Result<Vec<Interface>, Error> {
    let mut file = File::open(NET_DEV_FILE)
        .map_err(|_| ServiceError::MissingFileError(String::from(NET_DEV_FILE)))?;

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|_| ServiceError::MissingFileError(String::from(NET_DEV_FILE)))?;

    let file_content = str::from_utf8(&buffer);
    let split = file_content.unwrap().split("\n");

    let mut vec = split.collect::<Vec<&str>>();
    vec.drain(0..2);

    let mut interfaces: Vec<Interface> = Vec::new();

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
        interfaces.push(interface);
    }
    Ok(interfaces)
}
