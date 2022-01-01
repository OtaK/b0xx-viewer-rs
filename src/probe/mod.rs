use crate::{
    config::ViewerOptions,
    controllers::{ControllerState, ControllerType},
    ViewerError, ViewerResult,
};

pub mod gilrs;
pub mod serial;

#[cfg(feature = "fake_inputs")]
pub mod dummy;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ControllerMessage {
    State(ControllerState),
    Error(ViewerError),
    Reconnect,
    Quit,
}

pub trait ControllerProbe {
    fn new(config: &ViewerOptions) -> ViewerResult<Self>
    where
        Self: Sized;

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
