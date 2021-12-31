pub mod b0xx;
pub mod f1;

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

#[cfg(feature = "fake_serial")]
impl ControllerState {
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
