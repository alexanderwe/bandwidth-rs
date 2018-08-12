extern crate clap;
extern crate pretty_bytes;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;

mod config;
mod monitor;
mod proc;

use std::{thread, time};

use clap::{App, Arg, SubCommand};
use config::get_config;
use failure::Error;

pub fn get_monitor() -> Result<String, Error> {
    let cfg = get_config()?;

    let monitor = monitor::read_dev(&cfg)?;
    Ok(monitor)
}

pub fn monitor() {
    match get_monitor() {
        Ok(monitor) => println!("{}", monitor),
        Err(e) => println!("Monitor unavailable ({})", e),
    }
}

fn main() {
    let matches = App::new("bandwitdh-rs")
        .version("0.0.1")
        .author("Alexander Weiß")
        .about("Small tool to monitor your bandwitdh usage")
        .arg(
            Arg::with_name("config")
                .short("c")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("loop")
                .short("l")
                .help("Continously print the bandwitdh usage\nTo be used within the terminal"),
        )
        .get_matches();

    let looping: bool = matches.is_present("loop");

    if looping {
        monitor();
        thread::sleep(time::Duration::from_secs(1));
    } else {
        monitor();
    }
}
