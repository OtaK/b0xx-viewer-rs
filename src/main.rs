#![windows_subsystem = "windows"]

#[macro_use]
extern crate log;

mod b0xx_state;
mod cli;
#[macro_use]
mod config;
mod error;
mod serial_probe;
mod ui;

pub use self::error::*;

pub fn main() {
    pretty_env_logger::init();

    let options = cli::cli_options();

    let rx = match serial_probe::start_serial_probe() {
        Ok(rx) => rx,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    ui::start_gui(rx, options)
}
