use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crate::{
    config::ViewerOptions,
    controllers::ControllerType,
    probe::{ControllerMessage, ControllerProbe},
    SafeGilrsError, ViewerError, ViewerResult,
};

#[derive(Debug)]
pub struct GilrsControllerProbe {
    gamepad_id: gilrs::GamepadId,
    controller_type: ControllerType,
    thread_handle: Option<std::thread::JoinHandle<()>>,
    is_connected: Arc<AtomicBool>,
}

impl ControllerProbe for GilrsControllerProbe {
    fn new(_config: &ViewerOptions) -> ViewerResult<Self> {
        let gilrs = gilrs::Gilrs::new().map_err(SafeGilrsError::from)?;
        for (g_id, g) in gilrs.gamepads() {
            match <ControllerType as std::str::FromStr>::from_str(g.name()) {
                // Controller not matching criteria, continue
                Err(_) => continue,
                // Yay!
                Ok(controller_type) => {
                    let gamepad_id = g_id;
                    return Ok(Self {
                        gamepad_id,
                        controller_type,
                        thread_handle: None,
                        is_connected: Arc::new(true.into()),
                    });
                }
            }
        }

        // Nothing matched, bye
        Err(ViewerError::ControllerNotFound)
    }

    fn is_connected(&self) -> bool {
        self.thread_handle.is_some() && self.is_connected.load(Ordering::SeqCst)
    }

    fn connect(&mut self) -> ViewerResult<crossbeam_channel::Receiver<super::ControllerMessage>> {
        // Thread-local data copies
        let gamepad_id = self.gamepad_id;
        let controller_type = self.controller_type;
        let inner_is_connected = Arc::clone(&self.is_connected);

        let (tx, rx) = crossbeam_channel::bounded(1);
        let thread_hwnd = std::thread::spawn(move || {
            let gilrs = gilrs::Gilrs::new().unwrap();
            loop {
                let gamepad = gilrs.gamepad(gamepad_id);
                let is_connected = gamepad.is_connected();
                inner_is_connected.store(is_connected, Ordering::SeqCst);
                if !gamepad.is_connected() {
                    let _ = tx.send(ControllerMessage::Reconnect);
                    break;
                }

                let state = gamepad.state();
                let controller_state = controller_type.controller_state_from_gil_report(state);
                let state_msg = ControllerMessage::State(controller_state);
                if tx.send(state_msg).is_err() {
                    info!("Reconnection detected, exiting runloop");
                    break;
                }
                // TODO: Do we delay iterations? Polling rates are not crazy-high so if we can save CPU cycles
            }
        });

        self.thread_handle = Some(thread_hwnd);

        Ok(rx)
    }

    fn disconnect(&mut self) {
        if !self.is_connected() {
            return;
        }

        if let Some(thread_hwnd) = self.thread_handle.take() {
            let _ = thread_hwnd.join();
        }
    }

    fn controller_type(&self) -> crate::controllers::ControllerType {
        todo!()
    }
}
