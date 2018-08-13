extern crate bandwidth_monitor;
extern crate clap;

use clap::{App, Arg};

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
        .get_matches();

    match bandwidth_monitor::run(&matches) {
        Ok(()) => (),
        Err(e) => println!("Monitor unavailable ({})", e),
    };
}
