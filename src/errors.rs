use std::error::Error as StdError;
use std::fmt;
use std::io::{Error as IoError, ErrorKind as IoErrorKind};

use async_tungstenite::tungstenite::error::Error as TungsteniteError;
use futures::channel::mpsc::SendError;
use serde_json::Error as JsonError;

use crate::shard::error::Error as GatewayError;

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(IoError),
    Json(JsonError),
    Gateway(GatewayError),
    Tungstenite(TungsteniteError),
}

impl From<IoError> for Error {
    fn from(e: IoError) -> Self {
        Self::Io(e)
    }
}

impl From<JsonError> for Error {
    fn from(e: JsonError) -> Self {
        Self::Json(e)
    }
}

impl From<GatewayError> for Error {
    fn from(e: GatewayError) -> Self {
        Self::Gateway(e)
    }
}

impl From<TungsteniteError> for Error {
    fn from(e: TungsteniteError) -> Self {
        Self::Tungstenite(e)
    }
}

impl From<SendError> for Error {
    fn from(e: SendError) -> Self {
        Self::Io(IoError::new(std::io::ErrorKind::Other, e))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(inner) => fmt::Display::fmt(&inner, f),
            Self::Json(inner) => fmt::Display::fmt(&inner, f),
            Self::Tungstenite(inner) => fmt::Display::fmt(&inner, f),
            Self::Gateway(inner) => fmt::Display::fmt(&inner, f),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Io(inner) => Some(inner),
            Self::Json(inner) => Some(inner),
            Self::Tungstenite(inner) => Some(inner),
            Self::Gateway(inner) => Some(inner),
        }
    }
}

// might not be needed for most but
// i feel like this is a nice addition
impl From<Error> for IoError {
    fn from(e: Error) -> Self {
        IoError::new(IoErrorKind::Other, e)
    }
}
