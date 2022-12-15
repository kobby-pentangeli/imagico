use super::{DecodeArgs, EncodeArgs, PrintArgs, RemoveArgs};
use crate::{
    png::{chunk::Chunk, Png},
    ProgramResult,
};
use std::convert::TryFrom;
use std::fs;

/// Encode a message into a PNG file and save the result
pub fn encode(args: EncodeArgs) -> ProgramResult<()> {
    let infile_bytes = fs::read(&args.infile_path)?;
    let outfile = args.outfile_path.unwrap_or(args.infile_path);

    let mut png = Png::try_from(infile_bytes.as_slice())?;
    let chunk = Chunk::new(args.chunk_type, args.message.as_bytes().to_vec());
    png.append_chunk(chunk);
    fs::write(outfile, png.as_bytes())?;
    Ok(())
}

/// Search for a message hidden in a PNG file and print the message if one is found
pub fn decode(args: DecodeArgs) -> ProgramResult<()> {
    let file_bytes = fs::read(&args.file_path)?;
    let png = Png::try_from(file_bytes.as_slice())?;
    let chunk = png.chunk_by_type(&args.chunk_type.to_string());
    if let Some(val) = chunk {
        println!("{}", val);
    } else {
        println!("No hidden message found!");
    }
    Ok(())
}

/// Remove a chunk from a PNG file and save the result
pub fn remove(args: RemoveArgs) -> ProgramResult<()> {
    let file_bytes = fs::read(&args.file_path)?;
    let mut png = Png::try_from(file_bytes.as_slice())?;
    match png.remove_chunk(&args.chunk_type.to_string()) {
        Ok(chunk) => {
            fs::write(&args.file_path, png.as_bytes())?;
            println!("Removed chunk: {}", chunk);
        }
        Err(e) => println!("Error: {}", e),
    }
    Ok(())
}

/// Print all of the chunks in a PNG file
pub fn print_chunks(args: PrintArgs) -> ProgramResult<()> {
    let file_bytes = fs::read(&args.file_path)?;
    let png = Png::try_from(file_bytes.as_slice())?;
    for chunk in png.chunks() {
        println!("{}", chunk);
    }
    Ok(())
}
