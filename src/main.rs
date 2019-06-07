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
    let rx = match serial_probe::run_serial_probe() {
        Ok(rx) => rx,
        Err(e) => {
            error!("{}", e);
            return;
        }
    };

    loop {
        match rx.recv().map_err(Into::into) {
            Ok(Ok(state)) => {
                info!("{:#?}", state);
            }
            Ok(Err(e)) | Err(e) => {
                error!("{}", e);
                break;
            }
        }
    }
}
