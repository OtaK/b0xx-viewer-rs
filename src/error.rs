#[derive(Debug, thiserror::Error)]
/// This is needed to wrap Gilrs's `NotImplemented` error that carries around the Gilrs instance around,
/// making the error not `Send + Sync` and causes issues with `thiserror`
pub enum SafeGilrsError {
    #[error("Gilrs does not support current platform.")]
    NotImplemented,
    #[error("Either `pressed ≤ released` or one of values is outside [0.0, 1.0] range.")]
    InvalidAxisToBtn,
    #[error(transparent)]
    Other(Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl From<gilrs::Error> for SafeGilrsError {
    fn from(e: gilrs::Error) -> Self {
        match e {
            // Here we discard the gilrs::Gilrs instance to make the error Send + Sync
            gilrs::Error::NotImplemented(_) => Self::NotImplemented,
            gilrs::Error::InvalidAxisToBtn => Self::InvalidAxisToBtn,
            gilrs::Error::Other(ie) => Self::Other(ie),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ViewerError {
    #[error("A B0XX or Frame1 controller could not be found on your system. Are you sure it's connected through the USB port?")]
    ControllerNotFound,
    #[error("IoError: {0}")]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    GilrsError(#[from] SafeGilrsError),
    #[error(transparent)]
    SerialPortError(#[from] serialport::Error),
    #[error("Internal serial thread recv error: {0}")]
    SerialThreadRecvError(#[from] crossbeam_channel::RecvError),
    #[error(transparent)]
    SerialThreadSendError(#[from] crossbeam_channel::SendError<()>),
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
