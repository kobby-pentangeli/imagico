use crate::png::chunk_type::ChunkType;
use clap::{Args, Parser, Subcommand};
use core::str::FromStr;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct App {
    /// Optional name of argument
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
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

#[derive(Args)]
pub struct EncodeArgs {}

#[derive(Args)]
pub struct DecodeArgs {
    /// Path to the PNG file
    pub file_path: PathBuf,

    /// Chunk type (like "ruSt")
    #[arg(value_parser = ChunkType::from_str)]
    pub chunk_type: ChunkType,
}

#[derive(Args)]
pub struct RemoveArgs {
    /// Path to the PNG file
    pub file_path: PathBuf,

    /// Chunk type (like "ruSt")
    #[arg(value_parser = ChunkType::from_str)]
    pub chunk_type: ChunkType,
}

#[derive(Args)]
pub struct PrintArgs {
    /// Path to the PNG file
    pub file_path: PathBuf,
}
