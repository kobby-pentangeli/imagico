use crate::png::chunk_type::ChunkType;
use clap::{Args, Parser, Subcommand};
use core::str::FromStr;
use std::path::PathBuf;

mod commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct App {
    /// Optional name of argument
    name: Option<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Eq, PartialEq)]
pub enum Commands {
    /// Add a secret message to a PNG file
    Encode(EncodeArgs),

    /// Retrieve a secret message from a PNG file
    Decode(DecodeArgs),

    /// Remove a secret message from a PNG file
    Remove(RemoveArgs),

    /// Print all chunks in a PNG file
    Print(PrintArgs),
}

#[derive(Args, Debug, Eq, PartialEq)]
pub struct EncodeArgs {
    /// Path to the input PNG file
    pub infile_path: PathBuf,

    /// Chunk type (like "ruSt")
    #[arg(value_parser = ChunkType::from_str)]
    pub chunk_type: ChunkType,

    /// Your secret message
    pub message: String,

    /// Path to the output PNG file (optional)
    pub outfile_path: Option<PathBuf>,
}

#[derive(Args, Debug, Eq, PartialEq)]
pub struct DecodeArgs {
    /// Path to the PNG file
    pub file_path: PathBuf,

    /// Chunk type (like "ruSt")
    #[arg(value_parser = ChunkType::from_str)]
    pub chunk_type: ChunkType,
}

#[derive(Args, Debug, Eq, PartialEq)]
pub struct RemoveArgs {
    /// Path to the PNG file
    pub file_path: PathBuf,

    /// Chunk type (like "ruSt")
    #[arg(value_parser = ChunkType::from_str)]
    pub chunk_type: ChunkType,
}

#[derive(Args, Debug, Eq, PartialEq)]
pub struct PrintArgs {
    /// Path to the PNG file
    pub file_path: PathBuf,
}

// #[cfg(test)]
// mod test {
//     #[allow(unused_imports)]
//     use super::*;

//     #[test]
//     pub fn test_encode() {
//         let expected = EncodeArgs {
//             infile_path: PathBuf::from("/a/b/c"),
//             chunk_type: ChunkType::from_str("RuSt").unwrap(),
//             message: "Cryptic imagico encoder".to_string(),
//             outfile_path: None,
//         };

//         match &App::parse().command {
//             Some(Commands::Encode(actual)) => {
//                 assert_eq!(*actual, expected);
//             }
//             Some(_cmds) => {}
//             None => {}
//         }
//     }

//     #[test]
//     pub fn test_encode_with_output_file() {
//         let expected = EncodeArgs {
//             infile_path: PathBuf::from("/a/b/c"),
//             chunk_type: ChunkType::from_str("RuSt").unwrap(),
//             message: "Cryptic imagico encoder".to_string(),
//             outfile_path: Some(PathBuf::from("/output/file/path")),
//         };

//         match &App::parse().command {
//             Some(Commands::Encode(actual)) => {
//                 assert_eq!(*actual, expected);
//             }
//             Some(_cmds) => {}
//             None => {}
//         }
//     }

//     #[test]
//     pub fn test_decode() {
//         let expected = DecodeArgs {
//             file_path: PathBuf::from("/a/b/c"),
//             chunk_type: ChunkType::from_str("ImAg").unwrap(),
//         };

//         match &App::parse().command {
//             Some(Commands::Decode(actual)) => {
//                 assert_eq!(*actual, expected);
//             }
//             Some(_cmds) => {}
//             None => {}
//         }
//     }

//     #[test]
//     pub fn test_remove() {
//         let expected = RemoveArgs {
//             file_path: PathBuf::from("/a/b/c"),
//             chunk_type: ChunkType::from_str("imAG").unwrap(),
//         };

//         match &App::parse().command {
//             Some(Commands::Remove(actual)) => {
//                 assert_eq!(*actual, expected);
//             }
//             Some(_cmds) => {}
//             None => {}
//         }
//     }

//     #[test]
//     pub fn test_print() {
//         let expected = PrintArgs {
//             file_path: PathBuf::from("/a/b/c"),
//         };

//         match &App::parse().command {
//             Some(Commands::Print(actual)) => {
//                 assert_eq!(*actual, expected);
//             }
//             Some(_cmds) => {}
//             None => {}
//         }
//     }

//     #[test]
//     pub fn test_unknown_subcommand() {
//         todo!()
//     }
// }
