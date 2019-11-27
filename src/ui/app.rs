use crate::b0xx_state::B0xxState;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ViewerAppStatus {
    Running,
    Reconnecting,
    NeedsReconnection,
    Undefined
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

#[derive(Clone)]
pub struct ViewerApp {
    pub state: B0xxState,
    pub status: ViewerAppStatus,
    #[cfg(feature = "fps")]
    pub fps: fps_counter::FPSCounter,
}

impl std::fmt::Debug for ViewerApp {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.debug_struct("ViewerApp")
            .field("state", &self.state)
            .field("status", &self.status)
            .finish()
    }
}

impl Default for ViewerApp {
    fn default() -> Self {
        Self {
            state: B0xxState::default(),
            status: ViewerAppStatus::default(),
            #[cfg(feature = "fps")]
            fps: fps_counter::FPSCounter::new(),
        }
    }
}

impl ViewerApp {
    pub fn update_state(&mut self, new_state: B0xxState) -> bool {
        if self.state == new_state {
            return false;
        }

        self.state = new_state;
        true
    }
}
