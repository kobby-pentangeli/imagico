use super::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::{
    error::{ProgramError, ProgramResult},
    png::{chunk::Chunk, chunk_type::ChunkType, Png},
};
use std::convert::TryFrom;
use std::fs;
use std::str::FromStr;

/// Encode a message into a PNG file and save the result
pub fn encode(args: EncodeArgs) -> ProgramResult<()> {
    todo!()
}

/// Search for a message hidden in a PNG file and print the message if one is found
pub fn decode(args: DecodeArgs) -> ProgramResult<()> {
    todo!()
}

/// Remove a chunk from a PNG file and save the result
pub fn remove(args: RemoveArgs) -> ProgramResult<()> {
    todo!()
}

/// Print all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> ProgramResult<()> {
    todo!()
}
