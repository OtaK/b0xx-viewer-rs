# parallelograph

A multi-platform B0XX / Frame1 input viewer for your streaming needs

## Installation

Grab the latest release on the Releases page for your platform, and run the program from there.

## Usage

### Windows

Double click on the .exe file.

#### Passing arguments to a .exe file

If you wish to setup your input viewer (for example) to use the `--r1` start flag, removing B0XX r2 buttons:

* Right click on the .exe file, click on "Create shortcut"
* On the newly created shortcut, right click, go to "Properties"
* Add the desired flags after the full executable path in the "Target" field, like so:

![Properties Dialog](/assets/win_properties_dialog.png)

### Linux and macOS

Your user needs to be a member of the `dialout` group to access the serial ports exposed over USB.

To do so, run one of the following:

* `` sudo usermod -aG dialout `whoami` ``
* `` sudo gpasswd -a `whoami` dialout ``

Then download the executable, and launch it through a terminal.

### Options

Try launching the executable with `--help` to get all the current options
Note: This will not work on Windows because of platform restrictions

```text
parallelograph 0.7.0
Mathieu Amiot <amiot.mathieu@gmail.com>
GUI Viewer for B0XX/Frame1 controllers; particularly useful for streaming

USAGE:
    parallelograph_viewer.exe [OPTIONS]

OPTIONS:
    -a, --active <BTN_ACTIVE_COLOR>
            Sets a custom color for pressed/active buttons in hex format, eg. "#00FF00"

    -b, --background <BG_COLOR>
            Sets a custom background color in hex format, eg. "#00FF00"

    -c, --config <CONFIG>
            Sets the configuration file path

        --chromeless
            Makes the window chromeless - which means no borders, no titlebar, no close/minimize
            buttons etc

        --colored-rims
            Enables an alternative mode of inactive button coloring; Makes inactive button
            background neutral in favor of button rims instead

    -h, --help
            Print help information

    -i, --inactive <BTN_INACTIVE_COLOR>
            Sets a custom color for inactive buttons in hex format, eg. "#00FF00"

        --init-config
            Intializes an empty configuration in the executable's folder

    -j, --joystick-backend
            Uses system controller/joystick APIs to poll the controller state. Warning: You WILL
            lose input reporting accuracy because of it; for instance, this mode has no way of
            telling if a ModX/Y button is pressed if no accompanying direction isn't pressed

    -l, --labels
            Enable button labels

        --r1
            Disables B0XX r2 mode buttons for when you have a r1 B0XX

        --relax-arduino-detection
            Relaxes B0XX detection to allow any 16MHz Arduino-compatible device to connect

        --tty <TTY>
            Provide a custom COM port (Windows-only) or a /dev/ttyXXX path (Unix). Bypasses auto-
            detection, so proceed at your own risk!

    -V, --version
            Print version information

```

### Configuration file

A good example is in `cfg/gcc.toml`

You can create your own configuration file by launching the program with the `--init-config` option, then modify it with your favorite text editor!

## Building

Prequisites:

* Rust
* Linux-only: `libudev-dev`, `libxcb-shape0-dev`, `libxcb-xfixes0-dev`

Just `cargo build --release` and you should be good to go

### Fake Inputs Mode / Dev mode

There's a special feature that you can activate when building/running like so:

`cargo {run|build} --features fake_inputs [--release]`

It'll simulate state reports with completely random ones spaced by 170ms.

### Benchmark mode

`cargo run --features benchmark [--release]`

Starts the project with the `fake_inputs` and `fps` features to assess current rendering performance.

It currently stands at ~6500 fps (on a Ryzen 9 5950X / RTX3080 system running Windows 10).

### Enable Windows console for debugging/development

Because Windows is weird, you have a choice between displaying a window without a console, or both everytime (including just double-clicking a .exe file), I had to add a conditional feature to allow you to debug on Windows.

`cargo run --features win_console`

### Enable fps counter

Well, it's a, uh, FPS counter.

`cargo run --features fps`

## License

Licensed under either of these:

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   [https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or
   [https://opensource.org/licenses/MIT](https://opensource.org/licenses/MIT))

## Authors

* Mathieu "OtaK_" Amiot

I am not affiliated in any way with the following companies, and their IP remains their own:

* 20XX Inc. - Makers of the B0XX
* Frame1 LLC - Makers of the Frame1
