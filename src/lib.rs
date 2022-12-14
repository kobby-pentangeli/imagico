//! A library for manipulating PNG files.

#![warn(bad_style)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unused_results)]

/// Command-line arguments parsing
pub mod args;
/// Program-related errors and result type
pub mod error;
/// Implementation of `Chunk`, `ChunkType` and `Png`
pub mod png;
/// Utility functions
pub mod utils;
