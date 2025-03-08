use conrod_core::Color;

pub const DEFAULT_ACTIVE_COLOR: ViewerColor = ViewerColor(rgb::RGB8::new(0, 235, 255));
pub const DEFAULT_INACTIVE_COLOR: ViewerColor = ViewerColor(rgb::RGB8::new(85, 87, 83));
pub const DEFAULT_BACKGROUND_COLOR: ViewerColor = ViewerColor(rgb::RGB8::new(19, 19, 19));

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum ViewerColorType {
    Active,
    Inactive,
    Background,
}

#[derive(
    Debug, Clone, Copy, serde_with::SerializeDisplay, serde_with::DeserializeFromStr, PartialEq, Eq,
)]

pub struct ViewerColor(rgb::RGB8);

impl ViewerColor {
    pub const fn active_default() -> Self {
        DEFAULT_ACTIVE_COLOR
    }

    pub const fn inactive_default() -> Self {
        DEFAULT_INACTIVE_COLOR
    }

    pub const fn background_default() -> Self {
        DEFAULT_BACKGROUND_COLOR
    }
}

impl Default for ViewerColor {
    fn default() -> Self {
        Self::inactive_default()
    }
}

impl From<Color> for ViewerColor {
    fn from(value: Color) -> Self {
        let (r, g, b) = (
            (value.red() * 255.).ceil() as u8,
            (value.green() * 255.).ceil() as u8,
            (value.blue() * 255.).ceil() as u8,
        );

        ViewerColor(rgb::RGB8::new(r, g, b))
    }
}

impl Into<Color> for ViewerColor {
    fn into(self) -> Color {
        Color::Rgba(
            self.0.r as f32 / 255.,
            self.0.g as f32 / 255.,
            self.0.b as f32 / 255.,
            1.,
        )
    }
}

impl std::str::FromStr for ViewerColor {
    type Err = crate::error::ViewerOptionConfigError;

    fn from_str(s: &str) -> Result<Self, crate::error::ViewerOptionConfigError> {
        let color_value = u32::from_str_radix(s.trim_start_matches('#'), 16)?;
        let (r, g, b) = (
            ((color_value >> 16) & 255) as u8,
            ((color_value >> 8) & 255) as u8,
            (color_value & 255) as u8,
        );

        Ok(Self(rgb::RGB8::new(r, g, b)))
    }
}

impl std::fmt::Display for ViewerColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rgb::RGB8 { r, g, b } = self.0;
        write!(f, "#{r:02X}{g:02X}{b:02X}")
    }
}

impl std::ops::Deref for ViewerColor {
    type Target = rgb::RGB8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct ViewerButtonColors {
    #[serde(default)]
    pub start: ViewerColor,
    #[serde(default)]
    pub y: ViewerColor,
    #[serde(default)]
    pub x: ViewerColor,
    #[serde(default)]
    pub b: ViewerColor,
    #[serde(default)]
    pub a: ViewerColor,
    #[serde(default)]
    pub l: ViewerColor,
    #[serde(default)]
    pub r: ViewerColor,
    #[serde(default)]
    pub z: ViewerColor,
    #[serde(default)]
    pub up: ViewerColor,
    #[serde(default)]
    pub down: ViewerColor,
    #[serde(default)]
    pub right: ViewerColor,
    #[serde(default)]
    pub left: ViewerColor,
    #[serde(default)]
    pub mod_x: ViewerColor,
    #[serde(default)]
    pub mod_y: ViewerColor,
    #[serde(default)]
    pub c_left: ViewerColor,
    #[serde(default)]
    pub c_right: ViewerColor,
    #[serde(default)]
    pub c_up: ViewerColor,
    #[serde(default)]
    pub c_down: ViewerColor,
    #[serde(default)]
    pub mod_ls: ViewerColor,
    #[serde(default)]
    pub mod_ms: ViewerColor,
}

impl ViewerButtonColors {
    pub const fn default_inactive() -> Self {
        Self::new_with_color(DEFAULT_INACTIVE_COLOR)
    }

    pub const fn default_active() -> Self {
        Self::new_with_color(DEFAULT_ACTIVE_COLOR)
    }

    pub const fn new_with_color(color: ViewerColor) -> Self {
        Self {
            start: color,
            y: color,
            x: color,
            b: color,
            a: color,
            l: color,
            r: color,
            z: color,
            up: color,
            down: color,
            right: color,
            left: color,
            mod_x: color,
            mod_y: color,
            c_left: color,
            c_right: color,
            c_up: color,
            c_down: color,
            mod_ls: color,
            mod_ms: color,
        }
    }

    pub fn merge_defaults(&mut self, other: ViewerColor) {
        let default_color = ViewerColor::default();
        if self.start == default_color {
            self.start = other;
        }
        if self.y == default_color {
            self.y = other;
        }
        if self.x == default_color {
            self.x = other;
        }
        if self.b == default_color {
            self.b = other;
        }
        if self.a == default_color {
            self.a = other;
        }
        if self.l == default_color {
            self.l = other;
        }
        if self.r == default_color {
            self.r = other;
        }
        if self.z == default_color {
            self.z = other;
        }
        if self.up == default_color {
            self.up = other;
        }
        if self.down == default_color {
            self.down = other;
        }
        if self.right == default_color {
            self.right = other;
        }
        if self.left == default_color {
            self.left = other;
        }
        if self.mod_x == default_color {
            self.mod_x = other;
        }
        if self.mod_y == default_color {
            self.mod_y = other;
        }
        if self.c_left == default_color {
            self.c_left = other;
        }
        if self.c_right == default_color {
            self.c_right = other;
        }
        if self.c_up == default_color {
            self.c_up = other;
        }
        if self.c_down == default_color {
            self.c_down = other;
        }
        if self.mod_ls == default_color {
            self.mod_ls = other;
        }
        if self.mod_ms == default_color {
            self.mod_ms = other;
        }
    }
}
