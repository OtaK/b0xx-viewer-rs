use crate::{ViewerResult, ViewerOptionConfigError};
use crate::colors::*;

pub const DEFAULT_FILENAME: &str = "b0xx_viewer_config.toml";

fn hex_to_color(s: &str) -> Result<ViewerColor, ViewerOptionConfigError> {
    use std::str::FromStr as _;
    ViewerColor::from_str(s)
}

fn hex_to_button_colors(s: &str) -> Result<ViewerButtonColors, ViewerOptionConfigError> {
    let color = hex_to_color(s)?;
    Ok(ViewerButtonColors::new_with_color(color))
}

#[derive(Debug, Clone, PartialEq, Eq, clap::Parser, serde::Serialize, serde::Deserialize)]
#[command(author, version, about, long_about = None)]
pub struct ViewerOptions {
    /// Path of the current configuration. Used for caching purposes when saving/loading configurations
    #[serde(skip)]
    #[arg(skip)]
    path: std::path::PathBuf,
    /// Intializes an empty configuration in the executable's folder
    #[arg(long, exclusive(true))]
    #[serde(skip)]
    pub init_config: bool,
    /// Enable button labels
    #[arg(long = "labels", short)]
    #[serde(default)]
    pub display_labels: bool,
    /// Makes the window chromeless (i.e. removes window decorations such as titlebar, minimize/close buttons etc)
    #[arg(long)]
    #[serde(default)]
    pub chromeless: bool,
    /// Enables B0XX r1 mode to remove the 2 extra buttons
    #[arg(long = "r1")]
    #[serde(default)]
    pub is_r1_b0xx: bool,
    /// Enables an alternative mode of inactive button coloring; Makes inactive button background neutral in favor of button rims instead.
    #[arg(long)]
    #[serde(default)]
    pub colored_rims: bool,
    /// Relaxes B0XX detection to allow any 16MHz Arduino-compatible device to connect
    #[arg(long)]
    #[serde(default)]
    pub relax_arduino_detection: bool,
    /// Sets the configuration file path
    #[arg(long = "config", short = 'c')]
    #[serde(skip)]
    pub config_path: Option<std::path::PathBuf>,
    /// Sets a custom background color in hex format, eg. "#00FF00"
    #[arg(long = "background", short = 'b', value_parser = hex_to_color, default_value = "#131313")]
    #[serde(default = "ViewerColor::background_default")]
    pub background_color: ViewerColor,
    /// Sets a custom color for pressed/active buttons in hex format, eg. "#00FF00"
    #[arg(long = "active", short = 'a', value_parser = hex_to_button_colors, default_value = "#00EBFF")]
    #[serde(default = "ViewerButtonColors::default_active")]
    pub button_active_colors: ViewerButtonColors,
    /// Sets a custom color for inactive buttons in hex format, eg. "#00FF00"
    #[arg(long = "inactive", short = 'i', value_parser = hex_to_button_colors, default_value = "#555753")]
    #[serde(default = "ViewerButtonColors::default_inactive")]
    pub button_inactive_colors: ViewerButtonColors,
    /// Provide a custom COM port (Windows-only) or a /dev/ttyXXX path (Unix). Bypasses auto-detection, so proceed at your own risk!
    #[arg(long = "tty")]
    #[serde(rename = "tty")]
    pub custom_tty: Option<String>
}

impl Default for ViewerOptions {
    fn default() -> Self {
        Self {
            init_config: false,
            relax_arduino_detection: false,
            display_labels: false,
            chromeless: false,
            background_color: DEFAULT_BACKGROUND_COLOR,
            button_inactive_colors: ViewerButtonColors::new_with_color(DEFAULT_INACTIVE_COLOR),
            button_active_colors: ViewerButtonColors::new_with_color(DEFAULT_ACTIVE_COLOR),
            custom_tty: None,
            is_r1_b0xx: false,
            colored_rims: false,
            config_path: None,
            path: Default::default(),
        }
    }
}

impl ViewerOptions {
    fn get_cwd() -> ViewerResult<std::path::PathBuf> {
        let mut path = std::env::current_exe()?;
        path.set_file_name(DEFAULT_FILENAME);
        Ok(path)
    }

    /// Loads a configuration at `path`.
    /// If `path` isn't provided, it will default to the current working directory of the executable.
    pub fn load_config(path: Option<&std::path::PathBuf>) -> ViewerResult<Self> {
        let path = if let Some(path) = path.cloned().take() {
            path
        } else {
            Self::get_cwd()?
        };

        if !path.exists() {
            return Err(ViewerOptionConfigError::NotFound.into());
        }

        let str_buf = std::fs::read_to_string(&path)?;
        let mut ret: ViewerOptions =
            toml::from_str(&str_buf).map_err(ViewerOptionConfigError::from)?;

        ret.button_active_colors
            .merge_defaults(ViewerColor::active_default());

        ret.button_inactive_colors
            .merge_defaults(ViewerColor::inactive_default());

        ret.path = path;
        log::debug!("Loaded configuration: {ret:#?}");
        Ok(ret)
    }

    /// Saves the current configuration at the desired path
    pub fn save_config(&mut self, path: Option<&std::path::PathBuf>) -> ViewerResult<()> {
        let path = if let Some(path) = path.cloned().take() {
            path
        } else {
            Self::get_cwd()?
        };

        let toml_output = toml::to_string_pretty(self)
            .map_err(ViewerOptionConfigError::from)?;
        let _ = std::fs::write(path.clone(), toml_output.into_bytes())?;
        self.path = path;
        Ok(())
    }

    pub fn run() -> ViewerResult<Option<Self>> {
        use clap::Parser as _;
        // Get configuration from cli
        let cli_options = Self::parse();

        if cli_options.init_config {
            let mut ret = ViewerOptions::default();
            let _ = ret.save_config(None).unwrap();
            log::info!("configuration saved in ./b0xx_viewer_config.toml");
            return Ok(None);
        }

        let mut config = Self::load_config(cli_options.config_path.as_ref()).unwrap_or_default();
        config.merge(cli_options);

        if let Some(tty) = config.custom_tty.take() {
            config.custom_tty = serialport::available_ports()?.into_iter().find(|p| p.port_name == tty).map(move |_| String::from(tty));

            if config.custom_tty.is_none() {
                log::error!("Provided port not found or not connected to system");
            }
        }

        // Side effect for the serial probe thread.
        if config.relax_arduino_detection {
            std::env::set_var("RELAX_ARDUINO_DETECT", "true");
        }

        log::trace!("Configuration: {config:#?}");

        Ok(Some(config))
    }

    pub fn merge(&mut self, other: Self) {
        self.display_labels |= other.display_labels;
        self.chromeless |= other.chromeless;
        self.is_r1_b0xx |= other.is_r1_b0xx;
        self.colored_rims |= other.colored_rims;
        self.relax_arduino_detection |= other.relax_arduino_detection;

        if other.background_color != ViewerColor::background_default() {
            self.background_color = other.background_color;
        }

        if other.button_inactive_colors != ViewerButtonColors::default_inactive() {
            self.button_inactive_colors = other.button_inactive_colors;
        }

        if other.button_active_colors != ViewerButtonColors::default_active() {
            self.button_active_colors = other.button_active_colors;
        }

        self.custom_tty = self.custom_tty.clone().or(other.custom_tty);
    }
}
