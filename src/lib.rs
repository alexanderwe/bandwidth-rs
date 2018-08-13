extern crate clap;
extern crate pretty_bytes;
extern crate termion;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;

mod config;
mod monitor;
mod proc;

use failure::Error;
use std::{thread, time};

pub fn run(matches: &clap::ArgMatches) -> Result<(), Error> {
    let cfg = config::get_config(&matches)?;

    if cfg.looping {
        loop {
            monitor(&cfg);
            thread::sleep(time::Duration::from_secs(1));
            print!("{}", termion::cursor::Up(1));
            print!("{}", termion::clear::CurrentLine);
        }
    } else {
        monitor(&cfg);
    }

    Ok(())
}

pub fn monitor(cfg: &config::Config) {
    match monitor::read_dev(&cfg) {
        Ok(monitor) => println!("{}", monitor),
        Err(e) => println!("Monitor unavailable ({})", e),
    }
}
