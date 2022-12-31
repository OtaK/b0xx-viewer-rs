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
    pub mod_ls: bool,
    pub mod_ms: bool,
}

#[cfg(feature = "fake_inputs")]
impl B0xxState {
    pub fn random(rng: &mut rand::rngs::SmallRng) -> Self {
        use rand::RngCore as _;
        let mut arr = [0u8; 20];
        rng.fill_bytes(&mut arr);
        B0xxState {
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

impl TryFrom<&[B0xxReport]> for B0xxState {
    type Error = crate::error::ViewerError;

    fn try_from(value: &[B0xxReport]) -> Result<Self, Self::Error> {
        if value.len() < 20 {
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
            mod_ls: value[18].into(),
            mod_ms: value[19].into(),
        })
    }
}

impl From<[B0xxReport; 20]> for B0xxState {
    fn from(value: [B0xxReport; 20]) -> Self {
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
            mod_ls: value[18].into(),
            mod_ms: value[19].into(),
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
            mod_ls: value[18].into(),
            mod_ms: value[19].into(),
        }
    }
}
