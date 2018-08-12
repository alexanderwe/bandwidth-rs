extern crate pretty_bytes;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate failure;

extern crate libc;
extern crate sysctl;

mod config;
mod monitor;
mod proc;

use config::get_config;
use failure::Error;

pub fn get_monitor() -> Result<String, Error> {
    let cfg = get_config()?;

    let monitor = monitor::read_dev(&cfg)?;
    Ok(monitor)
}

fn main() {
    match get_monitor() {
        Ok(monitor) => println!("{}", monitor),
        Err(e) => println!("Monitor unavailable ({})", e),
    }
}
