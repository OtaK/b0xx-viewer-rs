use failure_derive::Fail;

macro_rules! from_error {
    ($type:ty, $target:ident, $targetvar:expr) => {
        impl From<$type> for $target {
            fn from(s: $type) -> Self {
                $targetvar(s.into())
            }
        }
    };
}

#[derive(Debug, Fail)]
pub enum ViewerError {
    #[fail(
        display = "A B0XX could not be found on your system. Are you sure it's connected through the USB port?"
    )]
    B0xxNotFound,
    #[fail(display = "IoError: {}", _0)]
    IoError(std::io::Error),
    #[fail(display = "SerialPortError: {}", _0)]
    SerialPortError(serialport::Error),
    #[fail(display = "Internal serial thread error: {}", _0)]
    SerialThreadError(std::sync::mpsc::RecvError),
    #[fail(display = "An unknown error occured, sorry")]
    UnknownError,
}

from_error!(serialport::Error, ViewerError, ViewerError::SerialPortError);
from_error!(std::io::Error, ViewerError, ViewerError::IoError);
from_error!(
    std::sync::mpsc::RecvError,
    ViewerError,
    ViewerError::SerialThreadError
);
