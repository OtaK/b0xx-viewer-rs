use crate::error::ViewerError;
use conrod_core::Color;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("The supplied config path could not be found on the filesystem.")]
    NotFound,
    #[error("DeserializationError: {0}")]
    DeserializationError(#[from] toml::de::Error),
    #[error("SerializationError: {0}")]
    SerializationError(#[from] toml::ser::Error),
}

#[macro_export]
macro_rules! hex_to_color {
    ($v:expr) => {{
        let (r, g, b) = (
            (($v >> 16) & 255) as u8,
            (($v >> 8) & 255) as u8,
            ($v & 255) as u8,
        );

        conrod_core::color::rgb_bytes(r, g, b).into()
    }};
}

pub const DEFAULT_FILENAME: &str = "b0xx_viewer_config.toml";

lazy_static! {
    pub static ref DEFAULT_ACTIVE_COLOR: ViewerColor = ViewerColor(rgb::RGB8::new(0, 235, 255));
    pub static ref DEFAULT_INACTIVE_COLOR: ViewerColor = ViewerColor(rgb::RGB8::new(85, 87, 83));
    pub static ref DEFAULT_BACKGROUND_COLOR: ViewerColor = ViewerColor(rgb::RGB8::new(19, 19, 19));
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum ViewerColorType {
    Active,
    Inactive,
    Background,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ViewerColor(rgb::RGB8);

impl ViewerColor {
    fn active_default() -> Self {
        *DEFAULT_ACTIVE_COLOR
    }

    fn inactive_default() -> Self {
        *DEFAULT_INACTIVE_COLOR
    }

    fn background_default() -> Self {
        *DEFAULT_BACKGROUND_COLOR
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

impl std::ops::Deref for ViewerColor {
    type Target = rgb::RGB8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
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
    fn default_inactive() -> Self {
        Self::new_with_color(*DEFAULT_INACTIVE_COLOR)
    }

    fn default_active() -> Self {
        Self::new_with_color(*DEFAULT_ACTIVE_COLOR)
    }

    pub fn new_with_color(color: ViewerColor) -> Self {
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ViewerOptions {
    #[serde(default)]
    pub display_labels: bool,
    #[serde(default)]
    pub chromeless: bool,
    #[serde(default = "ViewerColor::background_default")]
    pub background_color: ViewerColor,
    #[serde(default = "ViewerButtonColors::default_inactive")]
    pub button_inactive_colors: ViewerButtonColors,
    #[serde(default = "ViewerButtonColors::default_active")]
    pub button_active_colors: ViewerButtonColors,
    #[serde(default)]
    pub is_r2_b0xx: bool,
    pub custom_tty: Option<String>,
    #[serde(skip)]
    path: std::path::PathBuf,
}

impl Default for ViewerOptions {
    fn default() -> Self {
        Self {
            display_labels: false,
            chromeless: false,
            background_color: *DEFAULT_BACKGROUND_COLOR,
            button_inactive_colors: ViewerButtonColors::new_with_color(*DEFAULT_INACTIVE_COLOR),
            button_active_colors: ViewerButtonColors::new_with_color(*DEFAULT_ACTIVE_COLOR),
            custom_tty: None,
            is_r2_b0xx: false,
            path: Default::default(),
        }
    }
}

impl ViewerOptions {
    fn get_cwd() -> Result<std::path::PathBuf, ViewerError> {
        let mut path = std::env::current_exe()?;
        path.set_file_name(DEFAULT_FILENAME);
        Ok(path)
    }

    pub fn load(path: std::path::PathBuf) -> Result<Self, ViewerError> {
        if !path.exists() {
            return Err(ConfigError::NotFound.into());
        }

        let buf = std::fs::read(path.clone())?;
        let mut ret: ViewerOptions =
            toml::de::from_slice(&buf).map_err(|e| ViewerError::from(ConfigError::from(e)))?;

        ret.button_active_colors
            .merge_defaults(ViewerColor::active_default());

        ret.button_inactive_colors
            .merge_defaults(ViewerColor::inactive_default());

        ret.path = path;
        debug!("Loaded configuration: {:#?}", ret);
        Ok(ret)
    }

    pub fn load_cwd() -> Result<Self, ViewerError> {
        Self::load(Self::get_cwd()?)
    }

    pub fn save_to(&mut self, path: std::path::PathBuf) -> Result<(), ViewerError> {
        let buf = toml::ser::to_vec(self)
            .map_err(|e| ViewerError::from(ConfigError::SerializationError(e)))?;
        let _ = std::fs::write(path.clone(), buf)?;
        self.path = path;
        Ok(())
    }

    pub fn save_cwd(&mut self) -> Result<(), ViewerError> {
        self.save_to(Self::get_cwd()?)
    }
}
