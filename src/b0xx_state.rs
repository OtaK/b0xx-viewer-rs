//000000000000000000000000
//                         000000000000000000000000

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
    l: bool,
    start: bool,
    left: bool,
    down: bool,
    right: bool,
    c_up: bool,
    c_down: bool,
    c_left: bool,
    c_right: bool,
    a: bool,
    y: bool,
    r: bool,
    b: bool,
    x: bool,
    z: bool,
    up: bool,
    mod_x: bool,
    mod_y: bool,
}

impl From<&[B0xxReport]> for B0xxState {
    fn from(value: &[B0xxReport]) -> Self {
        let mut ret = Self::default();
        ret.start = value[0].into();
        ret.y = value[1].into();
        ret.x = value[2].into();
        ret.b = value[3].into();
        ret.a = value[4].into();
        ret.l = value[5].into();
        ret.r = value[6].into();
        ret.z = value[7].into();
        ret.up = value[8].into();
        ret.down = value[9].into();
        ret.right = value[10].into();
        ret.left = value[11].into();
        ret.mod_x = value[12].into();
        ret.mod_y = value[13].into();
        ret.c_left = value[14].into();
        ret.c_right = value[15].into();
        ret.c_up = value[16].into();
        ret.c_down = value[17].into();

        ret
    }
}
