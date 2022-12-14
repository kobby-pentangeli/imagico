//! Error types

/// Representing the various errors associated with this application.
#[derive(Debug, thiserror::Error)]
pub enum ProgramError {
    /// Error originating from string conversion from
    /// UTF-8 byte vector
    #[error("{0}")]
    StrFromUtf8Error(String),

    /// Error from `TryFrom` trait implementations
    #[error("{0}")]
    TryFromError(String),

    /// Error originating from manipulating PNG chunks
    #[error("{0}")]
    ChunkOperationError(String),
}

impl From<std::io::Error> for ProgramError {
    fn from(e: std::io::Error) -> Self {
        Self::TryFromError(e.to_string())
    }
}

impl From<std::string::FromUtf8Error> for ProgramError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::StrFromUtf8Error(e.to_string())
    }
}
