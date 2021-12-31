use crate::controllers::ControllerState;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ViewerAppStatus {
    Running,
    Reconnecting,
    NeedsReconnection,
    Undefined,
}

impl Default for ViewerAppStatus {
    fn default() -> Self {
        ViewerAppStatus::Undefined
    }
}

impl ViewerAppStatus {
    pub fn set_running(&mut self) {
        if *self != ViewerAppStatus::Running {
            *self = ViewerAppStatus::Running;
        }
    }
}

#[derive(Debug, Default)]
#[cfg_attr(not(feature = "fps"), derive(Clone))]
pub struct ViewerApp {
    pub state: ControllerState,
    pub status: ViewerAppStatus,
    pub is_draggable: bool,
    pub is_dragged: bool,
    #[cfg(feature = "fps")]
    pub fps: fps_counter::FPSCounter,
}

impl ViewerApp {
    pub fn update_state(&mut self, new_state: ControllerState) -> bool {
        if self.state == new_state {
            return false;
        }

        self.state = new_state;
        true
    }
}
