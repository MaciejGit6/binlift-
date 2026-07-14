//! Magic-byte sniffing + format dispatch.

use crate::error::{BinliftError, Result};
use crate::formats::elf::ElfParser;
use crate::formats::Parser;
use crate::model::{Binary, BinaryFormat};

const ELF_MAGIC: [u8; 4] = [0x7F, b'E', b'L', b'F'];
const PE_MAGIC: [u8; 2] = [b'M', b'Z'];
// Mach-O thin magics: 32/64-bit, both byte orders.
const MACHO_MAGICS: [[u8; 4]; 4] = [
    [0xFE, 0xED, 0xFA, 0xCE],
    [0xCE, 0xFA, 0xED, 0xFE],
    [0xFE, 0xED, 0xFA, 0xCF],
    [0xCF, 0xFA, 0xED, 0xFE],
];

/// Identify a format from its leading magic bytes.
pub fn detect(data: &[u8]) -> Result<BinaryFormat> {
    if data.starts_with(&ELF_MAGIC) {
        Ok(BinaryFormat::Elf)
    } else if data.starts_with(&PE_MAGIC) {
        Ok(BinaryFormat::Pe)
    } else if MACHO_MAGICS.iter().any(|m| data.starts_with(m)) {
        Ok(BinaryFormat::MachO)
    } else {
        Err(BinliftError::UnknownFormat(
            data.iter().take(4).copied().collect(),
        ))
    }
}

/// Detect the format and dispatch to the matching parser.
pub fn load(data: &[u8]) -> Result<Binary> {
    match detect(data)? {
        BinaryFormat::Elf => ElfParser::parse(data),
        BinaryFormat::Pe => Err(BinliftError::Unsupported("PE parsing".into())),
        BinaryFormat::MachO => Err(BinliftError::Unsupported("Mach-O parsing".into())),
    }
}