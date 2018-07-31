extern crate termion;

// Import the `clear` module.
use termion::{clear, cursor};

use std::fs::File;
use std::io::{Read, Result};
use std::str;
use std::{thread, time};

const NET_DEV_FILE: &'static str = "/proc/net/dev";

pub fn run() {
    read_dev();
}

#[derive(Debug)]
struct Interface {
    name: String,
    received_bytes: u32,
    received_packets: u32,
    received_errs: u32,
    received_drop: u32,
    received_fifo: u32,
    received_frame: u32,
    received_compressed: u32,
    received_multicast: u32,
    transmit_bytes: u32,
    transmit_packets: u32,
    transmit_errs: u32,
    transmit_drop: u32,
    transmit_fifo: u32,
    transmit_colls: u32,
    transmit_carrier: u32,
    transmit_compressed: u32,
}

pub fn read_dev() -> Result<()> {
    loop {
        let mut file = File::open(NET_DEV_FILE)?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let file_content = str::from_utf8(&buffer);
        let split = file_content.unwrap().split("\n");

        let mut vec = split.collect::<Vec<&str>>();
        vec.drain(0..2);

        for s in vec {
            if s.len() == 0 {
                continue;
            }
            let vars = s.split_whitespace().collect::<Vec<&str>>();

            println!(
                "{:?}",
                Interface {
                    name: vars[0].to_string(),
                    received_bytes: vars[1].to_string().parse::<u32>().unwrap(),
                    received_packets: vars[2].to_string().parse::<u32>().unwrap(),
                    received_errs: vars[3].to_string().parse::<u32>().unwrap(),
                    received_drop: vars[4].to_string().parse::<u32>().unwrap(),
                    received_fifo: vars[5].to_string().parse::<u32>().unwrap(),
                    received_frame: vars[6].to_string().parse::<u32>().unwrap(),
                    received_compressed: vars[7].to_string().parse::<u32>().unwrap(),
                    received_multicast: vars[8].to_string().parse::<u32>().unwrap(),
                    transmit_bytes: vars[9].to_string().parse::<u32>().unwrap(),
                    transmit_packets: vars[10].to_string().parse::<u32>().unwrap(),
                    transmit_errs: vars[11].to_string().parse::<u32>().unwrap(),
                    transmit_drop: vars[12].to_string().parse::<u32>().unwrap(),
                    transmit_fifo: vars[13].to_string().parse::<u32>().unwrap(),
                    transmit_colls: vars[14].to_string().parse::<u32>().unwrap(),
                    transmit_carrier: vars[15].to_string().parse::<u32>().unwrap(),
                    transmit_compressed: vars[16].to_string().parse::<u32>().unwrap(),
                }
            );
        }

        thread::sleep(time::Duration::from_secs(1));
        print!("{}{}", cursor::Up(1), clear::CurrentLine);
        print!("{}{}", cursor::Up(1), clear::CurrentLine);
    }

    Ok(())
}
