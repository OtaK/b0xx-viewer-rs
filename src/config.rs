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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct ViewerColor(rgb::RGB8);

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct ViewerButtonColors {
    pub start: ViewerColor,
    pub y: ViewerColor,
    pub x: ViewerColor,
    pub b: ViewerColor,
    pub a: ViewerColor,
    pub l: ViewerColor,
    pub r: ViewerColor,
    pub z: ViewerColor,
    pub up: ViewerColor,
    pub down: ViewerColor,
    pub right: ViewerColor,
    pub left: ViewerColor,
    pub mod_x: ViewerColor,
    pub mod_y: ViewerColor,
    pub c_left: ViewerColor,
    pub c_right: ViewerColor,
    pub c_up: ViewerColor,
    pub c_down: ViewerColor,
    pub mod_ls: ViewerColor,
    pub mod_ms: ViewerColor,
}

impl ViewerButtonColors {
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
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ViewerOptions {
    pub display_labels: Option<bool>,
    pub chromeless: Option<bool>,
    pub background_color: ViewerColor,
    pub button_inactive_colors: ViewerButtonColors,
    pub button_active_colors: ViewerButtonColors,
    pub is_r2_b0xx: Option<bool>,
    pub custom_tty: Option<String>,
    #[serde(skip)]
    path: std::path::PathBuf,
}

impl Default for ViewerOptions {
    fn default() -> Self {
        Self {
            display_labels: None,
            chromeless: None,
            background_color: *DEFAULT_BACKGROUND_COLOR,
            button_inactive_colors: ViewerButtonColors::new_with_color(*DEFAULT_INACTIVE_COLOR),
            button_active_colors: ViewerButtonColors::new_with_color(*DEFAULT_ACTIVE_COLOR),
            custom_tty: None,
            is_r2_b0xx: None,
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
