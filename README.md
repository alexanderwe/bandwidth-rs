# bandwitdh-rs

bandwitdh-rs is a small tool which displays the current upload/download rates and the sent/received bytes for a certain interface.

## Supported Platforms

* Linux


## Installation

__Todo__


## Depdendencies

* You need to have FontAwesome 4 installed

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
label = %output%
format = <label>
``` 

![Polybar Screenshot](/screenshot.png)

## Sidemarks

* I am still learning Rust so expect some parts of the code to be not as idomatic as it could be. 
* The binary size is quite huge for what it does right now since it uses some more feature-full crates like `clap`. Those crates are included since I am not sure how this application will mature and what features will be added.
