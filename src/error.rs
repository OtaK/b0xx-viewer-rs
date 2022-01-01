#[derive(Debug, thiserror::Error)]
pub enum ViewerError {
    #[error("A B0XX or Frame1 controller could not be found on your system. Are you sure it's connected through the USB port?")]
    ControllerNotFound,
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),
    #[cfg(feature = "gilrs_backend")]
    #[error(transparent)]
    GilrsError(#[from] gilrs::Error),
    #[cfg(feature = "serial_backend")]
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

pub type ViewerResult<T> = Result<T, ViewerError>;
