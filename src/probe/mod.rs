use crate::{ViewerResult, controllers::{ControllerState, ControllerType}, ViewerError, config::ViewerOptions};

#[cfg(feature = "serial_backend")]
pub mod serial;

#[cfg(feature = "gilrs_backend")]
pub mod gilrs;

#[cfg_attr(feature = "fake_serial", allow(dead_code))]
#[derive(Debug)]
pub enum ControllerMessage {
    State(ControllerState),
    Error(ViewerError),
    Reconnect,
    Quit,
}

pub trait ControllerProbe {
    fn new(config: ViewerOptions) -> ViewerResult<Self> where Self: Sized;
    fn controller_type(&self) -> ControllerType;

    fn is_connected(&self) -> bool;
    fn connect(&mut self) -> ViewerResult<crossbeam_channel::Receiver<ControllerMessage>>;
    fn disconnect(&mut self);

    fn reconnect(&mut self) -> ViewerResult<crossbeam_channel::Receiver<ControllerMessage>> {
        self.disconnect();
        use backoff::backoff::Backoff as _;
        let mut backoff = backoff::ExponentialBackoff::default();
        loop {
            if let Ok(new_rx) = self.connect() {
                return Ok(new_rx);
            }

            if let Some(backoff_duration) = backoff.next_backoff() {
                std::thread::sleep(backoff_duration);
            }
        }
    }
}
