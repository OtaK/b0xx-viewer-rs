pub mod b0xx;
pub mod f1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControllerType {
    B0XX,
    Frame1,
    DIYB0XX,
}

impl ControllerType {
    pub const fn device_name(&self) -> &str {
        match self {
            ControllerType::B0XX => "Arduino Leonardo",
            ControllerType::Frame1 => "Frame1",
            ControllerType::DIYB0XX => "Arduino",
        }
    }

    pub fn controller_state_from_gil_report(
        &self,
        state: &gilrs::ev::state::GamepadState,
    ) -> ControllerState {
        match self {
            ControllerType::B0XX | ControllerType::DIYB0XX => {
                ControllerState::from_b0xx_gilrs(state)
            }
            ControllerType::Frame1 => ControllerState::from_frame1_gilrs(state),
        }
    }
}

impl std::str::FromStr for ControllerType {
    type Err = crate::ViewerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(Self::B0XX.device_name()) {
            Ok(Self::B0XX)
        } else if s.contains(Self::Frame1.device_name()) {
            Ok(Self::Frame1)
        } else if s.contains(Self::DIYB0XX.device_name()) {
            Ok(Self::DIYB0XX)
        } else {
            Err(crate::ViewerError::ControllerNotFound)
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct ControllerState {
    pub start: bool,
    pub y: bool,
    pub x: bool,
    pub b: bool,
    pub a: bool,
    pub l: bool,
    pub r: bool,
    pub z: bool,
    pub up: bool,
    pub down: bool,
    pub right: bool,
    pub left: bool,
    pub mod_x: bool,
    pub mod_y: bool,
    pub c_left: bool,
    pub c_right: bool,
    pub c_up: bool,
    pub c_down: bool,
    pub mod_ls: bool,
    pub mod_ms: bool,
}

impl ControllerState {
    #[cfg(feature = "fake_inputs")]
    pub fn random() -> Self {
        use rand::RngCore as _;
        let mut rng = rand::thread_rng();
        let mut arr = [0u8; 20];
        rng.fill_bytes(&mut arr);
        ControllerState {
            start: arr[0] == 1,
            y: arr[1] == 1,
            x: arr[2] == 1,
            b: arr[3] == 1,
            a: arr[4] == 1,
            l: arr[5] == 1,
            r: arr[6] == 1,
            z: arr[7] == 1,
            up: arr[8] == 1,
            down: arr[9] == 1,
            right: arr[10] == 1,
            left: arr[11] == 1,
            mod_x: arr[12] == 1,
            mod_y: arr[13] == 1,
            c_left: arr[14] == 1,
            c_right: arr[15] == 1,
            c_up: arr[16] == 1,
            c_down: arr[17] == 1,
            mod_ls: arr[18] == 1,
            mod_ms: arr[19] == 1,
        }
    }
}
