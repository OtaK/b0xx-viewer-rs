use crate::config::*;
use crate::hex_to_color;
use clap::{clap_app, crate_authors, crate_description, crate_version};

pub fn cli_options() -> ViewerOptions {
    let matches = clap_app!(parallelograph =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg labels: -l --labels "Enable button labels")
        (@arg bg_color: -b --background +takes_value "Sets a custom background color in hex format, eg. \"#00FF00\"")
        (@arg btn_inactive_color: -i --inactive +takes_value "Sets a custom color for inactive buttons in hex format, eg. \"#00FF00\"")
        (@arg btn_active_color: -a --active +takes_value "Sets a custom color for pressed/active buttons in hex format, eg. \"#00FF00\"")
        (@arg init_config: --init_config "Intializes an empty configuration in the executable's folder")
        (@arg config: -c --config +takes_value "Sets the configuration file path")
        (@arg chromeless: --chromeless "Makes the window chromeless")
        (@arg tty: --tty +takes_value "Provide a custom COM port (Windows-only) or a /dev/ttyXXX path (Unix). Bypasses auto-detection, so proceed at your own risk!")
        (@arg relax_arduino: --relax_arduino_detection "Relaxes B0XX detection to allow any 16MHz Arduino-compatible device to connect")
        (@arg r1: --r1 "Disables B0XX r2 mode buttons for when you have a r1 B0XX")
        (@arg colored_rims: --colored_rims "Enables an alternative mode of inactive button coloring; Makes inactive button background neutral in favor of button rims instead.")
    )
    .get_matches();

    if matches.is_present("init_config") {
        let mut ret = ViewerOptions::default();
        let _ = ret.save_cwd().unwrap();
        info!("configuration saved in ./parallelograph_config.toml");
        std::process::exit(0);
    }

    if matches.is_present("relax_arduino") {
        std::env::set_var("RELAX_ARDUINO_DETECT", "true");
    }

    let mut ret = if let Some(config_path) = matches.value_of("config").take() {
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

    if matches.is_present("labels") {
        ret.display_labels = true;
    }

    if matches.is_present("chromeless") {
        ret.chromeless = true;
    }

    if matches.is_present("r1") {
        ret.is_r2_b0xx = false;
    }

    if matches.is_present("colored_rims") {
        ret.colored_rims = true;
    }

    if let Some(tty) = matches.value_of("tty").take() {
        if let Ok(ports) = serialport::available_ports() {
            ret.custom_tty = ports
                .into_iter()
                .find(|p| p.port_name == tty)
                .map(move |_| String::from(tty));

            if ret.custom_tty.is_none() {
                error!("Provided port not found or not connected to system");
            }
        } else {
            error!("No ports available on the system, cannot lookup");
        }
    }

    if let Some(Ok(bg)) = matches
        .value_of("bg_color")
        .take()
        .map(|s| u32::from_str_radix(s.trim_start_matches('#'), 16))
    {
        ret.background_color = hex_to_color!(bg);
    }

    if let Some(Ok(bg)) = matches
        .value_of("btn_inactive_color")
        .take()
        .map(|s| u32::from_str_radix(s.trim_start_matches('#'), 16))
    {
        ret.button_inactive_colors = ViewerButtonColors::new_with_color(hex_to_color!(bg));
    }

    if let Some(Ok(bg)) = matches
        .value_of("btn_active_color")
        .take()
        .map(|s| u32::from_str_radix(s.trim_start_matches('#'), 16))
    {
        ret.button_active_colors = ViewerButtonColors::new_with_color(hex_to_color!(bg));
    }

    trace!("Configuration: {:#?}", ret);

    ret
}
