//! Error handling for the chunking library.

use std::fmt;

/// Result type alias for this crate
pub type Result<T> = std::result::Result<T, ChunkingError>;

/// Errors that can occur during chunking operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChunkingError {
    /// Configuration validation error
    InvalidConfig(String),
    /// Input data error
    InvalidInput(String),
    /// Internal processing error
    ProcessingError(String),
    /// I/O related error
    IoError(String),
}

impl ChunkingError {
    /// Create a new InvalidConfig error
    pub fn invalid_config<S: Into<String>>(msg: S) -> Self {
        ChunkingError::InvalidConfig(msg.into())
    }

    /// Create a new InvalidInput error
    pub fn invalid_input<S: Into<String>>(msg: S) -> Self {
        ChunkingError::InvalidInput(msg.into())
    }

    /// Create a new ProcessingError
    pub fn processing_error<S: Into<String>>(msg: S) -> Self {
        ChunkingError::ProcessingError(msg.into())
    }

    /// Create a new IoError
    pub fn io_error<S: Into<String>>(msg: S) -> Self {
        ChunkingError::IoError(msg.into())
    }
}

impl fmt::Display for ChunkingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChunkingError::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
            ChunkingError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ChunkingError::ProcessingError(msg) => write!(f, "Processing error: {}", msg),
            ChunkingError::IoError(msg) => write!(f, "I/O error: {}", msg),
        }
    }
}

impl std::error::Error for ChunkingError {}

impl From<std::io::Error> for ChunkingError {
    fn from(err: std::io::Error) -> Self {
        ChunkingError::IoError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = ChunkingError::invalid_config("test message");
        assert_eq!(err.to_string(), "Invalid configuration: test message");
    }

    #[test]
    fn test_error_from_io() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let chunk_err: ChunkingError = io_err.into();
        
        match chunk_err {
            ChunkingError::IoError(_) => (),
            _ => panic!("Expected IoError"),
        }
    }
}