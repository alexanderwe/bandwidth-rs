# bandwitdh-rs

bandwitdh-rs is a small tool which displays the current upload/download rates and the sent/received bytes for a certain interface.

## Supported Platforms

* Linux

## Installation

By now it is only possible to git clone this repo and build the executable by yourself. I will add it to crates.io soon. 

### Dependencies

* Latest Rust stable for your platform

```
git clone https://github.com/alexanderwe/bandwidth-monitor.git bandwidth-monitor
cd bandwith-monitor
cargo build --release
``` 

After that you can move the executable `/target/release/` anywhere you want. 

## Depdendencies

* You need to have FontAwesome 4 installed to display to up and down arrow

## Usage

Create a `config.toml` file inside the directory of the binary, or anywhere else (then you need the `-c` option) and define the interface you want to monitor:

```
interface = "enp6s0"
looping = true
```

Then just execute it:

```
bandwitdh-rs 0.0.1
Alexander Weiß
Small tool to monitor your bandwitdh usage

USAGE:
    bandwidth-monitor [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c <FILE>        Sets a custom config file
```

## polybar

bandwidth-rs can also be used with polybar or any other status bar, but is only tested with polybar. To execute it with polybar just do not execute it
with the loop mode. Polybar will take care of the execution of the script every second.

Here you can see the configuration for the use with polybar:

```
modules-right = bandwidthmonitor memory....

[module/bandwidthmonitor]
type = custom/script
exec = path/to/bandwitdh-rs
# or exec = path/to/bandwidth-rs -c path/to/your/config
label = %output%
format = <label>
interval = 1 
``` 

![Polybar Screenshot](/assets/screenshot.png)

## Sidemarks

* I am still learning Rust so expect some parts of the code to be not as idomatic as it could be. 
* The binary size is quite huge for what it does right now since it uses some more feature-full crates like `clap`. Those crates are included since I am not sure how this application will mature and what features will be added.
