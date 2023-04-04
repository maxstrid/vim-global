use std::fmt;

#[derive(Debug, Copy, Clone)]
pub enum XError {
    DisplayConnectionError,
    WireProtocolFailed,
}

impl fmt::Display for XError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            XError::DisplayConnectionError => write!(f, "Couldn't connect to XDisplay server"),
            XError::WireProtocolFailed => write!(f, "Conversion to wire protocol format failed for XSendEvent"),
        }
    }
}

impl std::error::Error for XError {}
