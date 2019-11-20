use crate::config::*;
use crate::hex_to_color;
use clap::{clap_app, crate_authors, crate_description, crate_version};

pub fn cli_options() -> ViewerOptions {
    let matches = clap_app!(b0xx_viewer =>
        (version: crate_version!())
        (author: crate_authors!())
        (about: crate_description!())
        (@arg labels: -l --labels "Enable button labels")
        (@arg bg_color: -b --background +takes_value "Sets a custom background color")
        (@arg btn_inactive_color: -i --inactive +takes_value "Sets a custom color for inactive buttons")
        (@arg btn_active_color: -a --active +takes_value "Sets a custom color for pressed/active buttons")
        (@arg init_config: --init_config "Intializes an empty configuration in the executable's folder")
        (@arg config: -c --config "Sets the configuration file path")
        (@arg chromeless: --chromeless "Makes the window chromeless")
    )
    .get_matches();

    if matches.is_present("init_config") {
        let mut ret = ViewerOptions::default();
        let _ = ret.save_cwd().unwrap();
        return ret;
    }

    let mut ret = if let Some(config_path) = matches.value_of("config").take() {
        ViewerOptions::load(config_path.into()).unwrap_or_default()
    } else {
        ViewerOptions::load_cwd().unwrap_or_default()
    };

    if matches.is_present("labels") {
        ret.display_labels = true;
    }

    if matches.is_present("chromeless") {
        ret.chromeless = true;
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

    ret
}
