#![windows_subsystem = "windows"]

#[macro_use]
extern crate log;

#[macro_use]
extern crate conrod_winit;

mod b0xx_state;
mod error;
mod serial_probe;
mod ui;

pub use self::error::*;

pub fn main() {
    pretty_env_logger::init();
    let rx = match serial_probe::start_serial_probe() {
        Ok(rx) => rx,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    ui::start_gui(rx)
}
