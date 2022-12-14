use crc::{Crc, CRC_32_ISO_HDLC};

/// Valid bytes are represented by the characters A-Z or a-z
pub fn is_valid_byte(byte: u8) -> bool {
    byte.is_ascii() && byte.is_ascii_alphabetic()
}

pub fn crc_checksum(bytes: &[u8]) -> u32 {
    Crc::<u32>::new(&CRC_32_ISO_HDLC).checksum(bytes)
}
