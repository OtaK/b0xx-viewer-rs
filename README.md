# b0xx-viewer

A multi-platform B0XX input viewer for your streaming needs

## Installation

Grab the latest release on the Releases page for your platform, unzip it, and run the program from there`.

## Usage

### Windows

Double click on the .exe file.

### Linux and macOS

Download the executable, and launch it through a terminal.

### Options

Try launching the executable with `--help` to get all the current options

```text
b0xx_viewer 0.3.0
Mathieu Amiot <amiot.mathieu@gmail.com>
GUI Viewer for B0XX controllers; particularly useful for streaming

USAGE:
    b0xx_viewer.exe [FLAGS] [OPTIONS]

FLAGS:
        --chromeless                 Makes the window chromeless
    -h, --help                       Prints help information
        --init_config                Intializes an empty configuration in the executable's folder
    -l, --labels                     Enable button labels
        --r2                         Enables B0XX r2 mode to account for the 2 extra buttons
        --relax_arduino_detection    Relaxes B0XX detection to allow any 16MHz Arduino-compatible device to connect
    -V, --version                    Prints version information

OPTIONS:
    -b, --background <bg_color>            Sets a custom background color
    -a, --active <btn_active_color>        Sets a custom color for pressed/active buttons
    -i, --inactive <btn_inactive_color>    Sets a custom color for inactive buttons
    -c, --config <config>                  Sets the configuration file path
        --tty <tty>                        Provide a custom COM port (Windows-only) or a /dev/ttyXXX path (Unix).
                                           Bypasses auto-detection, so proceed at your own risk!
```

### Configuration file

A good example is in `cfg/gcc.toml`

You can create your own configuration file by launching the program with the `--init_config` option, then modify it with your favorite text editor!

## Building

Prequisites:

- Rust
- Linux-only: `libudev-dev`, `libxcb-shape0-dev`, `libxcb-xfixes0-dev`

Just `cargo build --release` and you should be good to go

### Fake Serial Mode / Dev mode

There's a special feature that you can activate when building/running like so:

`cargo {run|build} --features fake_serial [--release]`

It'll simulate state reports with completely random ones. It looks funky but it allows to test and also assess current GUI performance.

### Enable Windows console for debugging/development

Because Windows is weird, you have a choice between displaying a window without a console, or both everytime (including just double-clicking a .exe file), I had to add a conditional feature to allow you to debug on Windows.

`cargo run --features win_console`

### Enable fps counter

Well, it's a, uh, FPS counter.

`cargo run --features fps`

## Authors

- Mathieu "OtaK_" Amiot
- 20XX Inc. - Makers of the B0XX
