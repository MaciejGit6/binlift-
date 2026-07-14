//! ELF parser.

pub mod header;
pub mod sections;
pub mod symbols;

use crate::error::Result;
use crate::formats::Parser;
use crate::model::{Binary, BinaryFormat};

use header::ElfHeader;

/// Parser for 64-bit little-endian ELF binaries.
pub struct ElfParser;

impl Parser for ElfParser {
    fn parse(data: &[u8]) -> Result<Binary> {
        let hdr = ElfHeader::parse(data)?;
        let sections = sections::parse(data, &hdr)?;
        let symbols = symbols::parse(&sections)?;

        Ok(Binary {
            format: BinaryFormat::Elf,
            entry_point: hdr.entry,
            sections,
            symbols,
        })
    }
}