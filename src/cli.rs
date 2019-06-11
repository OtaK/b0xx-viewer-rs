use clap::{clap_app, crate_authors, crate_description, crate_version};
use conrod_core::Color;
use lazy_static::lazy_static;

macro_rules! hex_to_color {
    ($v:expr) => {{
        let (r, g, b) = (
            (($v >> 16) & 255) as u8,
            (($v >> 8) & 255) as u8,
            ($v & 255) as u8,
        );

        conrod_core::color::rgb_bytes(r, g, b)
    }};
}

lazy_static! {
    pub static ref DEFAULT_ACTIVE_COLOR: Color = conrod_core::color::rgb_bytes(0, 235, 255);
    pub static ref DEFAULT_INACTIVE_COLOR: Color = conrod_core::color::CHARCOAL;
    pub static ref DEFAULT_BACKGROUND_COLOR: Color = conrod_core::color::rgb_bytes(19, 19, 19);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewerButtonColors {
    pub start: Color,
    pub y: Color,
    pub x: Color,
    pub b: Color,
    pub a: Color,
    pub l: Color,
    pub r: Color,
    pub z: Color,
    pub up: Color,
    pub down: Color,
    pub right: Color,
    pub left: Color,
    pub mod_x: Color,
    pub mod_y: Color,
    pub c_left: Color,
    pub c_right: Color,
    pub c_up: Color,
    pub c_down: Color,
}

impl ViewerButtonColors {
    pub fn new_with_color(color: Color) -> Self {
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
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewerOptions {
    pub display_labels: bool,
    pub background_color: Color,
    pub button_inactive_colors: ViewerButtonColors,
    pub button_active_colors: ViewerButtonColors,
}

impl Default for ViewerOptions {
    fn default() -> Self {
        Self {
            display_labels: false,
            background_color: *DEFAULT_BACKGROUND_COLOR,
            button_inactive_colors: ViewerButtonColors::new_with_color(*DEFAULT_INACTIVE_COLOR),
            button_active_colors: ViewerButtonColors::new_with_color(*DEFAULT_ACTIVE_COLOR),
        }
    }
}

pub fn cli_options() -> ViewerOptions {
    let matches = clap_app!(b0xx_viewer =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg labels: -l --labels "Enable button labels")
        (@arg bg_color: -b --background +takes_value "Sets a custom background color")
        (@arg btn_inactive_color: -i --inactive +takes_value "Sets a custom color for inactive buttons")
        (@arg btn_active_color: -a --active +takes_value "Sets a custom color for pressed/active buttons")
    )
    .get_matches();

    let mut ret = ViewerOptions::default();

    if matches.is_present("labels") {
        ret.display_labels = true;
    }

    if let Some(Ok(bg)) = matches
        .value_of("bg_color")
        .map(|s| u32::from_str_radix(s.trim_start_matches('#'), 16))
    {
        ret.background_color = hex_to_color!(bg);
    }

    if let Some(Ok(bg)) = matches
        .value_of("btn_inactive_color")
        .map(|s| u32::from_str_radix(s.trim_start_matches('#'), 16))
    {
        ret.button_inactive_colors = ViewerButtonColors::new_with_color(hex_to_color!(bg));
    }

    if let Some(Ok(bg)) = matches
        .value_of("btn_active_color")
        .map(|s| u32::from_str_radix(s.trim_start_matches('#'), 16))
    {
        ret.button_active_colors = ViewerButtonColors::new_with_color(hex_to_color!(bg));
    }

    ret
}
