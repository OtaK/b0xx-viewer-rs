use std::convert::TryFrom;

use super::ControllerState;

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

impl TryFrom<&[B0xxReport]> for ControllerState {
    type Error = crate::error::ViewerError;

    fn try_from(value: &[B0xxReport]) -> Result<Self, Self::Error> {
        if value.len() < 20 {
            return Err(crate::error::ViewerError::MalformedSerialReport);
        }

        Ok(ControllerState {
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

impl From<[B0xxReport; 20]> for ControllerState {
    fn from(value: [B0xxReport; 20]) -> Self {
        ControllerState {
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

impl From<[B0xxReport; 25]> for ControllerState {
    fn from(value: [B0xxReport; 25]) -> Self {
        ControllerState {
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
