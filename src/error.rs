#[derive(Debug, thiserror::Error)]
pub enum ViewerError {
    #[error(
        "A B0XX could not be found on your system. Are you sure it's connected through the USB port?"
    )]
    B0xxNotFound,
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    SerialPortError(#[from] serialport::Error),
    #[error("Internal serial thread error: {0}")]
    SerialThreadError(#[from] crossbeam_channel::RecvError),
    #[error("Configuration error: {0}")]
    ConfigError(#[from] ViewerOptionConfigError),
    #[error("The state report transmitted over serial was malformed")]
    MalformedSerialReport,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
    #[error("An unknown error occured, sorry")]
    UnknownError,
}

#[derive(Debug, thiserror::Error)]
pub enum ViewerOptionConfigError {
    #[error("Could not parse hex number to its numerical components")]
    HexNumberParseError(#[from] std::num::ParseIntError),
    #[error("The supplied config path could not be found on the filesystem.")]
    NotFound,
    #[error("TOML Deserialization Error: {0}")]
    DeserializationError(#[from] toml::de::Error),
    #[error("TOML Serialization Error: {0}")]
    SerializationError(#[from] toml::ser::Error),
}

pub type ViewerResult<T> = Result<T, ViewerError>;
