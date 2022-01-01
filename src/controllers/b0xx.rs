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

#[allow(clippy::from_over_into)]
impl Into<bool> for B0xxReport {
    fn into(self) -> bool {
        matches!(self, B0xxReport::On)
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

impl ControllerState {
    // Ref: https://github.com/project-slippi/Ishiiruka/blob/slippi/Data/Sys/Config/Profiles/GCPad/B0XX.ini
    // Ref: https://github.com/project-slippi/Ishiiruka/blob/slippi/Data/Sys/Config/Profiles/GCPad/B0XX_Linux.ini
    // Ref: https://github.com/project-slippi/Ishiiruka/blob/slippi/Data/Sys/Config/Profiles/GCPad/B0XX_macOS.ini
    pub(crate) fn from_b0xx_gilrs(state: &gilrs::ev::state::GamepadState) -> Self {
        let mut c_state = Self::default();

        let axis_iter = state.axes().filter_map(|(code, data)| {
            let code = code.into_u32();
            if (0..=4).contains(&code) {
                Some((code, data.value()))
            } else {
                None
            }
        });

        for (axis_code, value) in axis_iter {
            match axis_code {
                0 => {
                    if value > 0. {
                        c_state.right = true;
                    } else if value < 0. {
                        c_state.left = true;
                    }

                    match value.abs() as u16 {
                        1..=7000 => c_state.mod_y |= true,
                        7001..=14000 => c_state.mod_x |= true,
                        _ => {}
                    }
                }
                1 => {
                    if value > 0. {
                        c_state.up = true;
                    } else if value < 0. {
                        c_state.down = true;
                    }

                    match value.abs() as u16 {
                        1..=12000 => c_state.mod_x |= true,
                        12001..=16000 => c_state.mod_y |= true,
                        _ => {}
                    }
                }
                2 if value > 0. => c_state.c_right = true,
                2 if value < 0. => c_state.c_left = true,
                3 if value > 0. => c_state.c_up = true,
                3 if value < 0. => c_state.c_down = true,
                4 if value > 24000. => c_state.mod_ms = true,
                4 if value > 12000. => c_state.mod_ls = true,
                _ => continue,
            }
        }

        let buttons_iter = state.buttons().filter_map(|(code, data)| {
            let code = code.into_u32();
            if (0..=11).contains(&code) {
                Some((code, data.is_pressed()))
            } else {
                None
            }
        });

        for (button_code, is_pressed) in buttons_iter {
            match button_code {
                0 => c_state.a = is_pressed,
                1 => c_state.b = is_pressed,
                2 => c_state.x = is_pressed,
                3 => c_state.y = is_pressed,
                4 => c_state.z = is_pressed,
                5 => c_state.l = is_pressed,
                6 => c_state.r = is_pressed,
                7 => c_state.start = is_pressed,
                8 if is_pressed => {
                    c_state.mod_x = true;
                    c_state.mod_y = true;
                    c_state.c_left = true;
                }
                9 if is_pressed => {
                    c_state.mod_x = true;
                    c_state.mod_y = true;
                    c_state.c_up = true;
                }
                10 if is_pressed => {
                    c_state.mod_x = true;
                    c_state.mod_y = true;
                    c_state.c_right = true;
                }
                11 if is_pressed => {
                    c_state.mod_x = true;
                    c_state.mod_y = true;
                    c_state.c_down = true;
                }
                _ => continue,
            }
        }

        c_state
    }
}
