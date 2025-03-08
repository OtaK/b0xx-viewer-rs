# [v0.7.1] - 08/03/2025 - Maintenance release

## Important note

As I do not belong to the SSBM community anymore, this release is UNTESTED on real hardware

## Software

* Updated dependencies
  * This is a tentative (untested) fix for Windows getting `TimedOut` errors (cf: https://github.com/serialport/serialport-rs/issues/29)

# [v0.7.0] - 29/07/2023 - B0XX r3 & r4 support

## Important note

As I do not belong to the SSBM community anymore, this release is UNTESTED on real hardware

## Software

* Updated dependencies
* Added support for the new B0XX VID/PIDs (cf: <https://github.com/SimpleControllers/b0xx-input-viewer/blob/9f38bb7dc10b186299a0526ae2402d7d3d9b297d/serial.js#L17-L22>)
* **[BREAKING]** The r2 mode (with the 2 extra buttons) is now enabled by default. The `--r2` flag has been removed in favor of a `--r1` flag.
  * This is because as there are now 4 revisions of the b0xx, the 1st revision is getting less and less common. This makes sense to default to the more common build.
* **[BREAKING]** The button colors file format has changed. It now uses hex colors instead of the very annoying [r, g, b] notation from before
  * See [cfg/gcc.toml](cfg/gcc.toml) for an example
* **[BREAKING]** the `fake_serial` feature has been renamed to `fake_inputs`
  * The performance of this mode has been multiplied by 10 as well. This results in accurate performance benchmarks now
* Fixed cross-display DPI handing. The window now updates its internal DPI correctly when moved between monitors of differing scale factors.

## Other

* With the improvement on the `fake_inputs` random inputs generation, we now have accurate performance measurements
  * On a system based on a Ryzen 5950X & RTX 3080, the viewer outputs more than 5000 fps (yes).
  * This is obviously disabled in a normal environment as VSync is enabled for normal releases.
  * If you want to test your system's performance, you can run the code with `cargo run --features benchmark --release`

## Known issues

* Resizing the window between monitors with different scaling factors (DPI) using the title bar is not behaving well
  * A fix for this is to use the `--chromeless` argument and use the Alt+Click dragging of the window to ensure moving the window works well.

## Other

* Improved & fixed CI warnings

# [v0.6.0] - 18/12/2021 - Window drag, licensing, edition 2021

## Important note

As I do not belong to the SSBM community anymore, this release is UNTESTED on real hardware

## Software

* Added Alt+Click window dragging to allow chromeless windows to be moved (#32)
* Added documentation for running the program properly on Linux without `sudo` (#30)
* Updated dependencies

## Other

* Changed license to dual Apache 2.0 / MIT (#33)
* Upgraded Rust to edition 2021 (#35)

# [v0.5.1] - 27/01/2021 - High DPI fix

## Software

* Fixed issue with handling high DPI screens
* Updated dependencies

# [v0.5.0] - 28/12/2020 - Button rims

## Software

* Added support for button rims coloring #24
* Updated dependencies

# [v0.4.6] - 17/11/2020 - Dual logger

## Software

* Added dual logger to have a text .log file (in the same folder b0xx-viewer is ran from) to still get logs/diagnostics in case of a crash.

# [v0.4.5] - 01/11/2020 - Button layout fix

## Software

* A long awaited fix to the button layout, making it more accurate as per the official B0XX blueprint.

# [v0.4.4] - 01/11/2020 - Performance and fixes

## Software

* Fixed performance issues by increasing it threefold (3f837eb)
* Changed CLI documentation thanks to feedback (thx Pipsqueak)

# [v0.4.3] - 30/10/2020 - Bugfix in CPU-bound situations

## Software

* Fixed an issue where, in high-CPU pressure situations, the buffer would shift forward and render incorrect button presses.

# [v0.4.2] - 20/10/2020 - Maintenance

## Software

* Update dependencies

# [v0.4.1] - 29/09/2020 - Support for r2 b0xx

## Software

* Fixes for configuration loading issues
* Added support for B0XX r2

# [v0.3.1] - 19/08/2020 - Bug fixes and dependencies updates

## Software

* Fixed a configuration issue where optional values wouldn't be parsed correctly
* Added exponential backoff for reconnection
* Removed `failure` in favor of `thiserror`

# [v0.3.0] - 27/11/2019 - Performance & QoL

## Software

* Fixed general performance, see #12 and #13
* Added a reconnection overlay UI, for the user to see if connection has been lost instead of just freezing
* Fixed Windows-specific buffer alignment bug, see #14 & #15
* Fixed label alignment within buttons in the UI
* Added partial support for B0XX r2
* Documentation improvements

# [v0.2.0] - 24/11/2019 - Support for UI config

## Software

* Added support for configuration files
* Added support for button colors & labels (active/inactive setting). You can find a sample config for colors mimicking a Gamecube Controller in the cfg folder. Other samples will be there.
* Added support for chromeless window. Sometimes having a chrome around the window doesn't play well with some recording/streaming software so I added the option to disable window decorations.
* Added support for relaxed Arduino detection. Useful especially for DIY B0XX devices that rely not only on Arduino Leonardo but other 16MHz MCUs.
* Added support of a custom TTY/COM port directive to force the software to connect to that port, bypassing completely autodetection.
* Small patches/optimizations here and there.

## Other

* Added continuous integration with GitHub actions. Any pull request/commit will be built on all 3 platforms (Windows, macOS, Linux), ensuring that nothing breaks too hard.
* Added automatic binary building and releasing with GitHub Actions for when I create a GitHub release. The following binaries were built with this system.

# [v0.1.0] - 10/06/2019

* Initial Public release
