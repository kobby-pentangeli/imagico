//! Implements `Chunk` as described by the PNG specification.

use crate::{
    chunk_type::ChunkType,
    error::{ProgramError, ProgramResult},
    utils::crc_checksum,
};
use std::io::{BufReader, Read};

const MAX_LEN: u32 = 2_u32.pow(31) - 1;

/// A validated PNG chunk. See the PNG Spec for more details
/// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    /// Creates a new `Chunk` from a `ChunkType` and chunk data
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Self {
        let chksm_bytes: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .copied()
            .chain(data.iter().cloned())
            .collect();
        let crc = crc_checksum(&chksm_bytes);

        Self {
            length: data.len() as u32,
            chunk_type,
            data,
            crc,
        }
    }

    /// The length of the data portion of this chunk.
    pub fn length(&self) -> u32 {
        self.length
    }

    /// The `ChunkType` of this chunk
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    /// The raw data contained in this chunk in bytes
    pub fn data(&self) -> &[u8] {
        &self.data
    }

    /// The CRC of this chunk
    pub fn crc(&self) -> u32 {
        self.crc
    }

    /// Returns the data stored in this chunk as a `String`.
    ///
    /// This function returns an error if the stored data is not valid UTF-8.
    pub fn data_as_string(&self) -> ProgramResult<String> {
        Ok(String::from_utf8(self.data().to_vec())?)
    }

    /// Returns this chunk as a byte sequences described by the PNG spec.
    ///
    /// The following data is included in this byte sequence in order:
    /// 1. Length of the data *(4 bytes)*
    /// 2. Chunk type *(4 bytes)*
    /// 3. The data itself *(`length` bytes)*
    /// 4. The CRC of the chunk type and data *(4 bytes)*
    pub fn as_bytes(&self) -> Vec<u8> {
        self.length()
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type().bytes().iter())
            .chain(self.data().iter())
            .chain(self.crc().to_be_bytes().iter())
            .copied()
            .collect::<Vec<u8>>()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = ProgramError;

    fn try_from(bytes: &[u8]) -> ProgramResult<Self> {
        if bytes.is_empty() {
            return Err("Chunk is empty".into());
        }

        let mut reader = BufReader::new(bytes);
        let mut buffer: [u8; 4] = [0; 4];

        // 1. Read the `length` bytes
        reader.read_exact(&mut buffer)?;
        let length = u32::from_be_bytes(buffer);
        if length > MAX_LEN {
            return Err("Length of data received exceeds MAX_LEN(2^31-1)".into());
        }

        // 2. Read the `chunk_type` bytes
        reader.read_exact(&mut buffer)?;
        let chunk_type = ChunkType::try_from(buffer)?;

        // 3. Read the `data` bytes
        let mut data: Vec<u8> = vec![0; length as usize];
        reader.read_exact(&mut data)?;
        if data.len() != length as usize {
            return Err("Length mismatch".into());
        }

        // 4. Read the `crc` bytes
        reader.read_exact(&mut buffer)?;
        let received_crc = u32::from_be_bytes(buffer);

        let chksm_bytes: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .copied()
            .chain(data.iter().cloned())
            .collect();
        let expected_crc = crc_checksum(&chksm_bytes);
        if expected_crc != received_crc {
            return Err(format!(
                "CRC mismatch: expected: {}, received: {}",
                expected_crc, received_crc
            )
            .into());
        }

        Ok(Self {
            length,
            chunk_type,
            data,
            crc: expected_crc,
        })
    }
}

impl core::fmt::Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
