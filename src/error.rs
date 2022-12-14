/// Representing returned results for various operations in this application.
pub type ProgramResult<T> = std::result::Result<T, ProgramError>;

/// Representing the various errors associated with this application.
#[derive(Debug, thiserror::Error)]
pub enum ProgramError {
    #[error("{0}")]
    StrFromUtf8Error(String),

    #[error("{0}")]
    TryFromError(String),

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
