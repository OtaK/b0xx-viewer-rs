# b0xx-viewer

A multi-platform B0XX input viewer for your streaming needs

## Installation

Grab the latest release on the Releases page for your platform.

## Usage

### Windows

Double click on the .exe file.

### Linux and macOS

Download the executable, and launch it through a terminal.

### Options

Try launching the executable with `--help` to get all the current options

```
b0xx_viewer 0.1.0
Mathieu Amiot <amiot.mathieu@gmail.com>
GUI Viewer for B0XX controllers; particularly useful for streaming

USAGE:
    b0xx_viewer [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --background <bg_color>            Sets a custom background color
    -a, --active <btn_active_color>        Sets a custom color for pressed/active buttons
    -i, --inactive <btn_inactive_color>    Sets a custom color for inactive buttons
```

## Building

Prequisites:
- Rust

Just `cargo build --release` and you should be good to go

### Fake Serial Mode / Dev mode

There's a special feature that you can activate when building/running like so:

`cargo {run|build} --features fake_serial [--release]`

It'll simulate state reports with completely random ones. It looks funky but it allows to test and also assess current GUI performance.

## Authors

- Mathieu "OtaK_" Amiot
- 20XX Inc. - Makers of the B0XX
