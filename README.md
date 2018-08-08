# bandwitdh-rs

bandwitdh-rs is a small tool which displays the current upload/download rates and the sent/received bytes for a certain interface.

## Supported Platforms

* Linux

## Installation

## Usage

Create a `config.toml` file inside the directory of the binary and define the interface you want to monitor:

```
interface = "enp6s0"
```

Then just execute it:

```
bandwidth-rs -l
```

## polybar

bandwidth-rs can also be used with polybar or any other status bar, but is only tested with polybar. To execute it with polybar just do not run it
in the loop mode, this will not work. Polybar will take care of the execution of the script every second.

Here you can see the configuration for the use with polybar:
```
modules-right = bandwidthmonitor memory....

[module/bandwidthmonitor]
type = custom/script
exec = path/to/script
label = %output%
format = <label>
``` 
