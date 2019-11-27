use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum B0xxReport {
    Off = b'0',
    On = b'1',
    End = b'\n',
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
            b'0' => B0xxReport::Off,
            b'1' => B0xxReport::On,
            b'\n' => B0xxReport::End,
            _ => B0xxReport::Invalid,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq)]
pub struct B0xxState {
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

impl TryFrom<&[B0xxReport]> for B0xxState {
    type Error = crate::error::ViewerError;

    fn try_from(value: &[B0xxReport]) -> Result<Self, Self::Error> {
        if value.len() < 18 {
            return Err(crate::error::ViewerError::MalformedSerialReport);
        }

        Ok(B0xxState {
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
        })
    }
}

impl From<[B0xxReport; 18]> for B0xxState {
    fn from(value: [B0xxReport; 18]) -> Self {
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

impl From<[B0xxReport; 25]> for B0xxState {
    fn from(value: [B0xxReport; 25]) -> Self {
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
