//! Error types for NYC Taxi data processing operations.

use std::fmt;

#[derive(Debug)]
pub enum ProcessingError {
    /// I/O operation failed.
    Io(std::io::Error),

    /// CSV parsing or serialization failed.
    Csv(csv::Error),

    /// Data validation failed.
    Validation { message: String },

    /// General processing error.
    Processing { message: String },

    /// JSON serialization/deserialization error.
    Json(serde_json::Error),
}

impl fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessingError::Io(err) => write!(f, "IO error: {}", err),
            ProcessingError::Csv(err) => write!(f, "CSV parsing error: {}", err),
            ProcessingError::Validation { message } => {
                write!(f, "Data validation error: {}", message)
            }
            ProcessingError::Processing { message } => write!(f, "Processing error: {}", message),
            ProcessingError::Json(err) => write!(f, "JSON error: {}", err),
        }
    }
}

impl std::error::Error for ProcessingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ProcessingError::Io(err) => Some(err),
            ProcessingError::Csv(err) => Some(err),
            ProcessingError::Json(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for ProcessingError {
    fn from(err: std::io::Error) -> Self {
        ProcessingError::Io(err)
    }
}

impl From<csv::Error> for ProcessingError {
    fn from(err: csv::Error) -> Self {
        ProcessingError::Csv(err)
    }
}

impl From<serde_json::Error> for ProcessingError {
    fn from(err: serde_json::Error) -> Self {
        ProcessingError::Json(err)
    }
}
