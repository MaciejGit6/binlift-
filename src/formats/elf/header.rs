//! ELF header parsing.

use crate::binfmt::Reader;
use crate::error::{BinliftError, Result};

const ELF_MAGIC: [u8; 4] = [0x7F, b'E', b'L', b'F'];
const ELFCLASS64: u8 = 2;
const ELFDATA2LSB: u8 = 1;

/// The fields we need from the ELF file header.
#[derive(Debug, Clone)]
pub struct ElfHeader {
    pub entry: u64,
    pub shoff: u64,     // section header table offset
    pub shentsize: u16, // size of one section header
    pub shnum: u16,     // number of section headers
    pub shstrndx: u16,  // index of the section-name string table
}

impl ElfHeader {
    /// Parse and validate a 64-bit little-endian ELF header.
    pub fn parse(data: &[u8]) -> Result<ElfHeader> {
        let mut r = Reader::new(data);

        let mut magic = [0u8; 4];
        for b in &mut magic {
            *b = r.read_u8()?;
        }
        if magic != ELF_MAGIC {
            return Err(BinliftError::UnknownFormat(magic.to_vec()));
        }

        if r.read_u8()? != ELFCLASS64 {
            return Err(BinliftError::Unsupported("only ELF64 is supported".into()));
        }
        if r.read_u8()? != ELFDATA2LSB {
            return Err(BinliftError::Unsupported(
                "only little-endian ELF is supported".into(),
            ));
        }

        // Skip the rest of e_ident (version, osabi, abiversion, padding).
        r.seek(16);

        let _e_type = r.read_u16_le()?;
        let _e_machine = r.read_u16_le()?;
        let _e_version = r.read_u32_le()?;
        let entry = r.read_u64_le()?;
        let _e_phoff = r.read_u64_le()?;
        let shoff = r.read_u64_le()?;
        let _e_flags = r.read_u32_le()?;
        let _e_ehsize = r.read_u16_le()?;
        let _e_phentsize = r.read_u16_le()?;
        let _e_phnum = r.read_u16_le()?;
        let shentsize = r.read_u16_le()?;
        let shnum = r.read_u16_le()?;
        let shstrndx = r.read_u16_le()?;

        Ok(ElfHeader {
            entry,
            shoff,
            shentsize,
            shnum,
            shstrndx,
        })
    }
}