use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Represents a Standard or FD CAN frame.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanFrame {
    pub id: u32,
    pub data: Vec<u8>,
    pub is_extended: bool,
    pub is_fd: bool,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinFrame {
    pub id: u8,
    pub data: Vec<u8>,
    pub timestamp: u64,
}

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AutoNexusError {
    #[error("Hardware error occurred")]
    HardwareError,
    #[error("Protocol error occurred")]
    ProtocolError,
    #[error("Operation timed out")]
    Timeout,
    #[error("Invalid data received")]
    InvalidData,
    #[error("Feature not supported")]
    NotSupported,
}

pub type AutoNexusResult<T> = Result<T, AutoNexusError>;
