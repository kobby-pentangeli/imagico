use crate::{
    chunk::Chunk,
    error::{ProgramError, ProgramResult},
};

pub struct Png {
    header: [u8; 8],
    chunks: Vec<Chunk>,
}

impl Png {
    pub const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71, 13, 10, 26, 10];

    pub fn from_chunks(chunks: Vec<Chunk>) -> Self {
        todo!()
    }

    pub fn append_chunk(&mut self, chunk: Chunk) {
        todo!()
    }

    pub fn remove_chunk(&mut self, chunk_type: &str) -> ProgramResult<Chunk> {
        todo!()
    }

    pub fn header(&self) -> &[u8; 8] {
        todo!()
    }

    pub fn chunks(&self) -> &[Chunk] {
        todo!()
    }

    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        todo!()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        todo!()
    }
}

impl TryFrom<&[u8]> for Png {
    type Error = ProgramError;

    fn try_from(value: &[u8]) -> ProgramResult<Self> {
        todo!()
    }
}

impl core::fmt::Display for Png {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{chunk_type::ChunkType, utils::PNG_FILE};
    use std::convert::TryFrom;
    use std::str::FromStr;

    fn testing_chunks() -> Vec<Chunk> {
        let mut chunks = Vec::new();

        chunks.push(chunk_from_strings("FrSt", "I am the first chunk").unwrap());
        chunks.push(chunk_from_strings("miDl", "I am another chunk").unwrap());
        chunks.push(chunk_from_strings("LASt", "I am the last chunk").unwrap());

        chunks
    }

    fn testing_png() -> Png {
        let chunks = testing_chunks();
        Png::from_chunks(chunks)
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
        let png = Png::try_from(&PNG_FILE[..]);
        assert!(png.is_ok());
    }

    #[test]
    fn test_as_bytes() {
        let png = Png::try_from(&PNG_FILE[..]).unwrap();
        let actual = png.as_bytes();
        let expected: Vec<u8> = PNG_FILE.iter().copied().collect();
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
