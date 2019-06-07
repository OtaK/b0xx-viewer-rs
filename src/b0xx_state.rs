#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum B0xxReport {
    Off = 0x30,
    On = 0x31,
    End = 0x0A,
    Invalid = 0x00,
}

impl Into<bool> for B0xxReport {
    fn into(self) -> bool {
        match self {
            B0xxReport::On => true,
            _ => false,
        }
    }
}

impl Default for B0xxReport {
    fn default() -> Self {
        B0xxReport::Invalid
    }
}

impl From<u8> for B0xxReport {
    fn from(value: u8) -> Self {
        match value {
            0x30 => B0xxReport::Off,
            0x31 => B0xxReport::On,
            0x0A => B0xxReport::End,
            _ => B0xxReport::Invalid,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct B0xxState {
    start: bool,
    y: bool,
    x: bool,
    b: bool,
    a: bool,
    l: bool,
    r: bool,
    z: bool,
    up: bool,
    down: bool,
    right: bool,
    left: bool,
    mod_x: bool,
    mod_y: bool,
    c_left: bool,
    c_right: bool,
    c_up: bool,
    c_down: bool,
}

#[cfg(feature = "fake_serial")]
impl B0xxState {
    pub fn random() -> Self {
        B0xxState {
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
        }
    }
}

impl From<&[B0xxReport]> for B0xxState {
    fn from(value: &[B0xxReport]) -> Self {
        B0xxState {
            start: value[0].into(),
            y: value[1].into(),
            x: value[2].into(),
            b: value[3].into(),
            a: value[4].into(),
            l: value[5].into(),
            r: value[6].into(),
            z: value[7].into(),
            up: value[8].into(),
            down: value[9].into(),
            right: value[10].into(),
            left: value[11].into(),
            mod_x: value[12].into(),
            mod_y: value[13].into(),
            c_left: value[14].into(),
            c_right: value[15].into(),
            c_up: value[16].into(),
            c_down: value[17].into(),
        }
    }
}
