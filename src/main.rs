#![cfg_attr(not(feature = "win_console"), windows_subsystem = "windows")]

mod b0xx_state;
mod colors;
mod config;
mod error;
pub use self::error::*;
mod serial_probe;
mod ui;
mod logger;

pub use self::error::*;

pub fn main() -> ViewerResult<()> {
    if let Ok(env) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", format!("b0xx_viewer=info,{env}"));
    } else {
        std::env::set_var("RUST_LOG", "b0xx_viewer=info");
    }

    let mut logger = logger::Logger::new();
    logger.init();

    let Some(options) = config::ViewerOptions::run()? else {
        std::process::exit(0);
    };

    let rx = serial_probe::start_serial_probe(&options.custom_tty)?;

    log::info!("Serial probe up and running");
    ui::start_gui(rx, options);
    Ok(())
}
