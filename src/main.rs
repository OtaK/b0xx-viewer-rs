#![cfg_attr(not(feature = "win_console"), windows_subsystem = "windows")]

#[macro_use]
extern crate log;

mod controllers;
mod cli;
#[macro_use]
mod config;
mod error;
mod serial_probe;
mod ui;
mod logger;

pub use self::error::*;

pub fn main() {
    if let Ok(env) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", format!("parallelograph_viewer=info,{}", env));
    } else {
        std::env::set_var("RUST_LOG", "parallelograph_viewer=info");
    }

    let mut logger = logger::Logger::new();
    logger.init();

    let options = cli::cli_options();

    let rx = match serial_probe::start_serial_probe(&options.custom_tty) {
        Ok(rx) => rx,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    info!("Serial probe up and running");
    ui::start_gui(rx, options)
}
