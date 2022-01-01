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

    #[cfg(feature = "gilrs_backend")]
    pub fn controller_state_from_report(&self, state: &gilrs::ev::state::GamepadState) -> ControllerState {
        match self {
            ControllerType::B0XX => ControllerState::from_b0xx_gilrs(state),
            ControllerType::Frame1 => todo!(),
            ControllerType::DIYB0XX => todo!(),
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
    #[cfg(feature = "fake_serial")]
    pub fn random() -> Self {
        ControllerState {
            start: rand::random::<bool>(),
            y: rand::random::<bool>(),
            x: rand::random::<bool>(),
            b: rand::random::<bool>(),
            a: rand::random::<bool>(),
            l: rand::random::<bool>(),
            r: rand::random::<bool>(),
            z: rand::random::<bool>(),
            up: rand::random::<bool>(),
            down: rand::random::<bool>(),
            right: rand::random::<bool>(),
            left: rand::random::<bool>(),
            mod_x: rand::random::<bool>(),
            mod_y: rand::random::<bool>(),
            c_left: rand::random::<bool>(),
            c_right: rand::random::<bool>(),
            c_up: rand::random::<bool>(),
            c_down: rand::random::<bool>(),
            mod_ls: rand::random::<bool>(),
            mod_ms: rand::random::<bool>(),
        }
    }
}
