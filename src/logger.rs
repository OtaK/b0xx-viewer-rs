use fern::colors::{Color, ColoredLevelConfig};

#[derive(Debug)]
pub struct Logger {
    has_init: bool,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            has_init: false,
        }
    }

    pub fn init(&mut self) {
        if self.has_init {
            return;
        }

        fern::Dispatch::new()
            .chain(stdout())
            .chain(filelog())
            .apply()
            .unwrap();

        self.has_init = true;
    }
}

fn stdout() -> fern::Dispatch {
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::White)
        .debug(Color::White)
        .trace(Color::BrightBlack)
        .info(Color::Green);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{target}] {level} > {message}",
                level = colors.color(record.level()),
                target = record.target(),
                message = message,
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
}

fn filelog() -> fern::Dispatch {
    let mut logpath = std::env::current_exe().unwrap();
    let _ = logpath.set_extension("log");

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] > {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.target(),
                message,
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file(logpath).unwrap())
}
