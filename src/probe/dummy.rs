use crate::controllers::ControllerState;

use super::{ControllerMessage, ControllerProbe};

#[derive(Debug)]
pub struct DummyControllerProbe;

impl ControllerProbe for DummyControllerProbe {
    fn new(_config: &crate::config::ViewerOptions) -> crate::ViewerResult<Self>
    where
        Self: Sized,
    {
        Ok(Self)
    }

    fn controller_type(&self) -> crate::controllers::ControllerType {
        crate::controllers::ControllerType::B0XX
    }

    fn is_connected(&self) -> bool {
        true
    }

    fn connect(
        &mut self,
    ) -> crate::ViewerResult<crossbeam_channel::Receiver<super::ControllerMessage>> {
        let (tx, rx) = crossbeam_channel::bounded(1);
        let sleep_dur = std::time::Duration::from_micros(8700);
        std::thread::spawn(move || loop {
            let _ = tx.send(ControllerMessage::State(ControllerState::random()));
            #[cfg(not(feature = "benchmark"))]
            std::thread::sleep(sleep_dur);
        });

        Ok(rx)
    }

    fn disconnect(&mut self) {}
}
