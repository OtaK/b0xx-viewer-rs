#![cfg_attr(not(feature = "win_console"), windows_subsystem = "windows")]
// Needed because fake_input mode bypasses a bunch of code and we really don't care
#![cfg_attr(
    feature = "fake_inputs",
    allow(unused_imports, unused_variables, dead_code)
)]

#[macro_use]
extern crate log;

mod cli;
mod config;
mod controllers;
mod error;
mod logger;
mod probe;
mod ui;

use crate::probe::{gilrs::GilrsControllerProbe, serial::SerialControllerProbe, ControllerProbe};

pub use self::error::*;

pub fn main() -> ViewerResult<()> {
    if let Ok(env) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", format!("parallelograph_viewer=info,{}", env));
    } else {
        std::env::set_var("RUST_LOG", "parallelograph_viewer=info");
    }

    let mut logger = logger::Logger::new();
    logger.init();

    let options = cli::cli_options();

    #[cfg(not(feature = "fake_inputs"))]
    let mut probe: Box<dyn ControllerProbe> = if options.joystick_api_backend {
        Box::new(GilrsControllerProbe::new(&options)?)
    } else {
        Box::new(SerialControllerProbe::new(&options)?)
    };
    #[cfg(feature = "fake_inputs")]
    let mut probe: Box<dyn ControllerProbe> =
        Box::new(crate::probe::dummy::DummyControllerProbe::new(&options)?);

    let rx = probe.connect()?;
    info!("Serial probe up and running");
    ui::start_gui(&mut probe, rx, options);

    probe.disconnect();

    Ok(())
}
