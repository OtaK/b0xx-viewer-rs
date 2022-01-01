use crate::controllers::ControllerType;
use crate::{ViewerResult, ViewerError};
use super::{ControllerProbe, ControllerMessage};

#[derive(Debug)]
pub struct GilrsControllerProbe {
    gamepad_id: gilrs::GamepadId,
    gilrs: gilrs::Gilrs,
    controller_type: ControllerType,
    thread_handle: Option<std::thread::JoinHandle<()>>,
}

impl ControllerProbe for GilrsControllerProbe {
    fn new(config: crate::config::ViewerOptions) -> crate::ViewerResult<Self> {
        let gilrs = gilrs::Gilrs::new()?;
        for (g_id, g) in gilrs.gamepads() {
            let g_name = g.name();
            match <ControllerType as std::str::FromStr>::from_str(g.name()) {
                // Controller not matching criteria, continue
                Err(_) => continue,
                // Yay!
                Ok(controller_type) => {
                    let gamepad_id = g.id();
                    return Ok(Self {
                        gamepad_id,
                        controller_type,
                        gilrs,
                        thread_handle: None,
                    });
                },
            }
        }

        // Nothing matched, bye
        Err(ViewerError::ControllerNotFound)
    }

    fn is_connected(&self) -> bool {
        self.thread_handle.is_some()
    }

    fn connect(&mut self) -> crate::ViewerResult<crossbeam_channel::Receiver<super::ControllerMessage>> {
        let gamepad = self.gilrs.gamepad(self.gamepad_id);
        let (tx, rx) = crossbeam_channel::bounded(1);
        // FIXME: This doens't work to move the gamepad to a thread, hmmmmmm (*slavic music intensifies*)
        // let thread_hwnd = std::thread::spawn(move || {
        //     loop {
        //         let state = gamepad.state();
        //         let controller_state = self.controller_type.controller_state_from_report(state);
        //         if tx.send(ControllerMessage::State(controller_state.into())).is_err() {
        //             info!("Reconnection detected, exiting runloop");
        //             break;
        //         }
        //     }
        // });
        // self.thread_handle = Some(thread_hwnd);

        Ok(rx)
    }

    fn disconnect(&mut self) {
        todo!()
    }

    fn controller_type(&self) -> crate::controllers::ControllerType {
        todo!()
    }
}
