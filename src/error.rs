#[derive(Debug, thiserror::Error)]
pub enum ViewerError {
    #[error("A B0XX could not be found on your system. Are you sure it's connected through the USB port?")]
    B0xxNotFound,
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    SerialPortError(#[from] serialport::Error),
    #[error("Internal serial thread error: {0}")]
    SerialThreadError(#[from] crossbeam_channel::RecvError),
    #[error("Configuration error: {0}")]
    ConfigError(#[from] crate::config::ConfigError),
    #[error("The state report transmitted over serial was malformed")]
    MalformedSerialReport,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error("An unknown error occured, sorry")]
    UnknownError,
}
