use crate::config::*;
use crate::ViewerResult;

fn parse_color(arg: &str) -> ViewerResult<ViewerColor> {
    let color_as_u32 = u32::from_str_radix(arg.trim_start_matches('#'), 16)
        .map_err(|_| ConfigError::InvalidHexColor)?;

    let (r, g, b) = (
        ((color_as_u32 >> 16) & 255) as u8,
        ((color_as_u32 >> 8) & 255) as u8,
        (color_as_u32 & 255) as u8,
    );

    Ok(ViewerColor::new(r, g, b))
}

#[derive(clap::Parser, Debug)]
#[clap(about, version, author)]
struct CliArgs {
    #[clap(short, long)]
    /// Enable button labels
    labels: bool,
    #[clap(short = 'b', long = "background", parse(try_from_str = parse_color))]
    /// Sets a custom background color in hex format, eg. "#00FF00"
    bg_color: Option<ViewerColor>,
    #[clap(short = 'i', long = "inactive", parse(try_from_str = parse_color))]
    /// Sets a custom color for inactive buttons in hex format, eg. "#00FF00"
    btn_inactive_color: Option<ViewerColor>,
    #[clap(short = 'a', long = "active", parse(try_from_str = parse_color))]
    /// Sets a custom color for pressed/active buttons in hex format, eg. "#00FF00"
    btn_active_color: Option<ViewerColor>,
    #[clap(long = "init-config")]
    /// Intializes an empty configuration in the executable's folder
    init_config: bool,
    #[clap(short, long)]
    /// Sets the configuration file path
    config: Option<String>,
    #[clap(long)]
    /// Makes the window chromeless - which means no borders, no titlebar, no close/minimize buttons etc.
    chromeless: bool,
    #[clap(long)]
    /// Provide a custom COM port (Windows-only) or a /dev/ttyXXX path (Unix). Bypasses auto-detection, so proceed at your own risk!
    tty: Option<String>,
    #[clap(long = "relax-arduino-detection")]
    /// Relaxes B0XX detection to allow any 16MHz Arduino-compatible device to connect
    relax_arduino: bool,
    #[clap(long)]
    /// Disables B0XX r2 mode buttons for when you have a r1 B0XX
    r1: bool,
    #[clap(short, long = "joystick-backend")]
    /// Uses system controller/joystick APIs to poll the controller state.
    /// Warning: You WILL lose input reporting accuracy because of it;
    /// for instance, this mode has no way of telling if a ModX/Y button
    /// is pressed if no accompanying direction isn't pressed
    joystick_api_backend: bool,
    #[clap(long = "colored-rims")]
    /// Enables an alternative mode of inactive button coloring; Makes inactive button background neutral in favor of button rims instead.
    colored_rims: bool,
}

pub fn cli_options() -> ViewerOptions {
    use clap::StructOpt as _;
    let mut cli_args = CliArgs::parse();

    if cli_args.init_config {
        let mut ret = ViewerOptions::default();
        let _ = ret.save_cwd().unwrap();
        info!("configuration saved in ./parallelograph_config.toml");
        std::process::exit(0);
    }

    if cli_args.relax_arduino {
        std::env::set_var("RELAX_ARDUINO_DETECT", "true");
    }

    let mut ret = if let Some(config_path) = cli_args.config.take() {
        info!("Loading custom config at {}", config_path);
        match ViewerOptions::load(config_path.into()) {
            Ok(config) => config,
            Err(e) => {
                error!("{}", e);
                error!("Falling back to default configuration");
                ViewerOptions::default()
            }
        }
    } else {
        ViewerOptions::load_cwd().unwrap_or_default()
    };

    if cli_args.labels {
        ret.display_labels = true;
    }

    if cli_args.chromeless {
        ret.chromeless = true;
    }

    if cli_args.r1 {
        ret.is_r2_b0xx = false;
    }

    if cli_args.colored_rims {
        ret.colored_rims = true;
    }

    if cli_args.joystick_api_backend {
        ret.joystick_api_backend = true;
    } else if let Some(tty) = cli_args.tty.take() {
        if let Ok(ports) = serialport::available_ports() {
            ret.custom_tty = ports
                .into_iter()
                .find(|p| p.port_name == tty)
                .map(move |_| tty);

            if ret.custom_tty.is_none() {
                error!("Provided port not found or not connected to system");
            }
        } else {
            error!("No ports available on the system, cannot lookup");
        }
    }

    if let Some(bg) = cli_args.bg_color {
        ret.background_color = bg;
    }

    if let Some(bg) = cli_args.btn_inactive_color {
        ret.button_inactive_colors = ViewerButtonColors::new_with_color(bg);
    }

    if let Some(bg) = cli_args.btn_active_color {
        ret.button_active_colors = ViewerButtonColors::new_with_color(bg);
    }

    trace!("Configuration: {:#?}", ret);

    ret
}
