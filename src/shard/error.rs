use std::error::Error as StdError;
use std::fmt;

use async_tungstenite::tungstenite::protocol::CloseFrame;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum Error {
    Closed(Option<CloseFrame<'static>>),
    ExpectedHello,
    HeartbeatFailed,
    InvalidAuthentication,
    InvalidHandshake,
    InvalidOpCode,
    ReconnectFailure,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Closed(frame) => {
                if let Some(CloseFrame { code, reason }) = frame {
                    write!(f, "Connection closed with code {code}: {reason}")
                } else {
                    write!(f, "Connection closed")
                }
            }
            Self::ExpectedHello => write!(f, "Expected a Hello"),
            Self::HeartbeatFailed => write!(f, "Failed sending a heartbeat"),
            Self::InvalidAuthentication => write!(f, "Sent invalid authentication"),
            Self::InvalidHandshake => write!(f, "Expected a valid Handshake"),
            Self::InvalidOpCode => write!(f, "Invalid OpCode"),
            Self::ReconnectFailure => write!(f, "Failed to Reconnect"),
        }
    }
}

impl StdError for Error {}

// pub mod close_frames {
//     const UNKNOWN_ERROR: u16 = 4000;
//     const INVALID_AUTH: u16 = 4001;
//     const IDENTIFY_TIMEOUT: u16 = 4002;
//     const UNKNOWN_OPCODE: u16 = 4004;
//     const INVALID_PAYLOAD: u16 = 4005;
//     const BAD_ROUTE: u16 = 4006;
//     const OUT_OF_SYNC: u16 = 4007;
// }
