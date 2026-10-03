use pyo3::exceptions::{PyFileNotFoundError, PyIOError, PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use std::fmt;

#[derive(Debug)]
pub enum WinevtxError {
    WindowsApi(String, u32),
    Io(std::io::Error),
    FileNotFound(String),
    XmlParse(String),
    EvtxParse(String),
    InvalidFormat(String),
    Other(String),
}

impl fmt::Display for WinevtxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WinevtxError::WindowsApi(msg, code) => {
                write!(f, "Windows Event Log API error {}: {}", code, msg)
            }
            WinevtxError::Io(err) => write!(f, "I/O error: {}", err),
            WinevtxError::FileNotFound(path) => write!(f, "File not found: {}", path),
            WinevtxError::XmlParse(msg) => write!(f, "XML parse error: {}", msg),
            WinevtxError::EvtxParse(msg) => write!(f, "EVTX parse error: {}", msg),
            WinevtxError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            WinevtxError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for WinevtxError {}

impl From<std::io::Error> for WinevtxError {
    fn from(err: std::io::Error) -> Self {
        WinevtxError::Io(err)
    }
}

impl From<evtx::err::EvtxError> for WinevtxError {
    fn from(err: evtx::err::EvtxError) -> Self {
        WinevtxError::EvtxParse(err.to_string())
    }
}

impl From<WinevtxError> for PyErr {
    fn from(err: WinevtxError) -> PyErr {
        match err {
            WinevtxError::FileNotFound(path) => {
                PyFileNotFoundError::new_err(format!("File not found: {}", path))
            }
            WinevtxError::InvalidFormat(msg) => PyValueError::new_err(msg),
            WinevtxError::Io(err) => PyIOError::new_err(err.to_string()),
            _ => PyRuntimeError::new_err(err.to_string()),
        }
    }
}

pub type Result<T> = std::result::Result<T, WinevtxError>;
