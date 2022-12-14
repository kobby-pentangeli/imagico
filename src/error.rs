/// Representing the various errors associated with this application.
pub type ProgramError = Box<dyn std::error::Error>;

/// Representing returned results for various operations in this application.
pub type ProgramResult<T> = std::result::Result<T, ProgramError>;
