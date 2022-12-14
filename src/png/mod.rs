//! Implements `Png` as described by the PNG specification.

pub mod chunk;
pub mod chunk_type;

use crate::error::{ProgramError, ProgramResult};
use core::str::FromStr;
use std::io::{BufReader, Read};

use chunk::Chunk;
use chunk_type::ChunkType;

/// A PNG container as described by the PNG spec
/// http://www.libpng.org/pub/png/spec/1.2/PNG-Contents.html
#[derive(Debug)]
pub struct Png {
    header: [u8; 8],
    chunks: Vec<Chunk>,
}

impl Png {
    /// The first eight bytes of a PNG file,
    /// which always contain the following (decimal) values:
    pub const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

    /// Creates a `Png` from a list of chunks using the correct header
    pub fn from_chunks(chunks: Vec<Chunk>) -> Self {
        Self {
            header: Self::STANDARD_HEADER,
            chunks,
        }
    }

    /// Appends a chunk to the end of this `Png` file's `Chunk` list.
    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    /// Searches for a `Chunk` with the specified `chunk_type` and removes the first
    /// matching `Chunk` from this `Png` list of chunks.
    pub fn remove_chunk(&mut self, chunk_type: &str) -> ProgramResult<Chunk> {
        let chunk_type = ChunkType::from_str(chunk_type)?;
        if let Some(pos) = self
            .chunks
            .iter()
            .position(|c| *c.chunk_type() == chunk_type)
        {
            Ok(self.chunks.remove(pos))
        } else {
            Err(ProgramError::ChunkOperationError(
                "No matching Chunk found for chunk type".to_string(),
            ))
        }
    }

    /// The header of this PNG.
    pub fn header(&self) -> &[u8; 8] {
        &self.header
    }

    /// Lists the `Chunk`s stored in this `Png`
    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    /// Searches for a `Chunk` with the specified `chunk_type` and returns the first
    /// matching `Chunk` from this `Png`.
    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        match ChunkType::from_str(chunk_type) {
            Ok(cht) => self.chunks.iter().find(|c| *c.chunk_type() == cht),
            Err(_) => None,
        }
    }

    /// Returns this `Png` as a byte sequence.
    /// These bytes will contain the header followed by the bytes of all of the chunks.
    pub fn as_bytes(&self) -> Vec<u8> {
        let chunks = self
            .chunks
            .iter()
            .flat_map(|x| x.as_bytes())
            .collect::<Vec<u8>>();
        self.header.iter().chain(chunks.iter()).copied().collect()
    }
}

impl TryFrom<&[u8]> for Png {
    type Error = ProgramError;

    fn try_from(bytes: &[u8]) -> ProgramResult<Self> {
        let mut reader = BufReader::new(bytes);

        // 1. Read the standard header
        let mut header = [0u8; 8];
        reader.read_exact(&mut header)?;
        if header != Self::STANDARD_HEADER {
            return Err(
                ProgramError::TryFromError(format!("Received header doesn't match with STANDARD HEADER; received: {:?}, expected: {:?}",
                header, Self::STANDARD_HEADER))
            );
        }

        // 2. Iterate through the list of chunks and process each chunk
        let mut chunks = vec![];
        let mut data_len_buf = [0u8; 4];
        // reads:
        // ** `length` == 4 bytes
        // ** `chunk_type` == 4 bytes
        // ** `data` == `length` bytes
        // ** `crc` == 4 bytes
        while let Ok(()) = reader.read_exact(&mut data_len_buf) {
            let chunk_position = 4 + u32::from_be_bytes(data_len_buf) + 4;
            let mut chunk_buf = vec![0; chunk_position as usize];
            reader.read_exact(&mut chunk_buf)?;
            let chained_bytes = data_len_buf
                .iter()
                .copied()
                .chain(chunk_buf.into_iter())
                .collect::<Vec<u8>>();

            let chunk = Chunk::try_from(chained_bytes.as_slice())?;
            chunks.push(chunk);
        }

        Ok(Self::from_chunks(chunks))
    }
}

impl core::fmt::Display for Png {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "STANDARD_HEADER: {:#?}", self.header())?;
        for chunk in self.chunks() {
            write!(f, "{}", chunk)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::DICE_PNG;
    use std::convert::TryFrom;
    use std::str::FromStr;

    fn testing_png() -> Png {
        let chunks = testing_chunks();
        Png::from_chunks(chunks)
    }

    fn testing_chunks() -> Vec<Chunk> {
        let mut chunks = Vec::new();

        chunks.push(chunk_from_strings("FrSt", "I am the first chunk").unwrap());
        chunks.push(chunk_from_strings("miDl", "I am another chunk").unwrap());
        chunks.push(chunk_from_strings("LASt", "I am the last chunk").unwrap());

        chunks
    }

    fn chunk_from_strings(chunk_type: &str, data: &str) -> ProgramResult<Chunk> {
        let chunk_type = ChunkType::from_str(chunk_type)?;
        let data: Vec<u8> = data.bytes().collect();

        Ok(Chunk::new(chunk_type, data))
    }

    #[test]
    fn test_from_chunks() {
        let chunks = testing_chunks();
        let png = Png::from_chunks(chunks);

        assert_eq!(png.chunks().len(), 3);
    }

    #[test]
    fn test_valid_from_bytes() {
        let chunk_bytes: Vec<u8> = testing_chunks()
            .into_iter()
            .flat_map(|chunk| chunk.as_bytes())
            .collect();

        let bytes: Vec<u8> = Png::STANDARD_HEADER
            .iter()
            .chain(chunk_bytes.iter())
            .copied()
            .collect();

        let png = Png::try_from(bytes.as_ref());

        assert!(png.is_ok());
    }

    #[test]
    fn test_invalid_header() {
        let chunk_bytes: Vec<u8> = testing_chunks()
            .into_iter()
            .flat_map(|chunk| chunk.as_bytes())
            .collect();

        let bytes: Vec<u8> = [13, 80, 78, 71, 13, 10, 26, 10]
            .iter()
            .chain(chunk_bytes.iter())
            .copied()
            .collect();

        let png = Png::try_from(bytes.as_ref());

        assert!(png.is_err());
    }

    #[test]
    fn test_invalid_chunk() {
        let mut chunk_bytes: Vec<u8> = testing_chunks()
            .into_iter()
            .flat_map(|chunk| chunk.as_bytes())
            .collect();

        #[rustfmt::skip]
        let mut bad_chunk = vec![
            0, 0, 0, 5,         // length
            32, 117, 83, 116,   // Chunk Type (bad)
            65, 64, 65, 66, 67, // Data
            1, 2, 3, 4, 5       // CRC (bad)
        ];

        chunk_bytes.append(&mut bad_chunk);

        let png = Png::try_from(chunk_bytes.as_ref());

        assert!(png.is_err());
    }

    #[test]
    fn test_list_chunks() {
        let png = testing_png();
        let chunks = png.chunks();
        assert_eq!(chunks.len(), 3);
    }

    #[test]
    fn test_chunk_by_type() {
        let png = testing_png();
        let chunk = png.chunk_by_type("FrSt").unwrap();
        assert_eq!(&chunk.chunk_type().to_string(), "FrSt");
        assert_eq!(&chunk.data_as_string().unwrap(), "I am the first chunk");
    }

    #[test]
    fn test_append_chunk() {
        let mut png = testing_png();
        png.append_chunk(chunk_from_strings("TeSt", "Message").unwrap());
        let chunk = png.chunk_by_type("TeSt").unwrap();
        assert_eq!(&chunk.chunk_type().to_string(), "TeSt");
        assert_eq!(&chunk.data_as_string().unwrap(), "Message");
    }

    #[test]
    fn test_remove_chunk() {
        let mut png = testing_png();
        png.append_chunk(chunk_from_strings("TeSt", "Message").unwrap());
        png.remove_chunk("TeSt").unwrap();
        let chunk = png.chunk_by_type("TeSt");
        assert!(chunk.is_none());
    }

    #[test]
    fn test_png_from_image_file() {
        let png = Png::try_from(&DICE_PNG[..]);
        assert!(png.is_ok());
    }

    #[test]
    fn test_as_bytes() {
        let png = Png::try_from(&DICE_PNG[..]).unwrap();
        let actual = png.as_bytes();
        let expected: Vec<u8> = DICE_PNG.iter().copied().collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_png_trait_impls() {
        let chunk_bytes: Vec<u8> = testing_chunks()
            .into_iter()
            .flat_map(|chunk| chunk.as_bytes())
            .collect();

        let bytes: Vec<u8> = Png::STANDARD_HEADER
            .iter()
            .chain(chunk_bytes.iter())
            .copied()
            .collect();

        let png: Png = TryFrom::try_from(bytes.as_ref()).unwrap();

        let _png_string = format!("{}", png);
    }
}
