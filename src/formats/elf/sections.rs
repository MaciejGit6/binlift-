//! ELF section parsing.

use crate::binfmt::Reader;
use crate::error::Result;
use crate::formats::elf::header::ElfHeader;
use crate::model::{Section, SectionPerms};

const SHT_NOBITS: u32 = 8; // .bss-style: occupies no space in the file

const SHF_WRITE: u64 = 0x1;
const SHF_ALLOC: u64 = 0x2;
const SHF_EXECINSTR: u64 = 0x4;

/// One raw ELF64 section header (only the fields we use).
struct RawSection {
    name_off: u32,
    sh_type: u32,
    flags: u64,
    addr: u64,
    offset: u64,
    size: u64,
}

/// Build normalized `Section`s from the section header table.
pub fn parse(data: &[u8], hdr: &ElfHeader) -> Result<Vec<Section>> {
    let raw = parse_raw(data, hdr)?;

    // The section-name string table is itself a section, at index shstrndx.
    let strtab: &[u8] = raw
        .get(hdr.shstrndx as usize)
        .map(|s| slice(data, s.offset, s.size))
        .unwrap_or(&[]);

    let mut sections = Vec::with_capacity(raw.len());
    for s in &raw {
        let body = if s.sh_type == SHT_NOBITS {
            Vec::new()
        } else {
            slice(data, s.offset, s.size).to_vec()
        };
        sections.push(Section {
            name: str_at(strtab, s.name_off),
            addr: s.addr,
            size: s.size,
            perms: perms_from_flags(s.flags),
            data: body,
        });
    }
    Ok(sections)
}

fn parse_raw(data: &[u8], hdr: &ElfHeader) -> Result<Vec<RawSection>> {
    let mut out = Vec::with_capacity(hdr.shnum as usize);
    for i in 0..hdr.shnum as u64 {
        let mut r = Reader::new(data);
        r.seek((hdr.shoff + i * hdr.shentsize as u64) as usize);

        let name_off = r.read_u32_le()?;
        let sh_type = r.read_u32_le()?;
        let flags = r.read_u64_le()?;
        let addr = r.read_u64_le()?;
        let offset = r.read_u64_le()?;
        let size = r.read_u64_le()?;
        // sh_link, sh_info, sh_addralign, sh_entsize follow — unused here.

        out.push(RawSection {
            name_off,
            sh_type,
            flags,
            addr,
            offset,
            size,
        });
    }
    Ok(out)
}

fn perms_from_flags(flags: u64) -> SectionPerms {
    SectionPerms {
        read: flags & SHF_ALLOC != 0,
        write: flags & SHF_WRITE != 0,
        execute: flags & SHF_EXECINSTR != 0,
    }
}

/// A bounds-safe sub-slice; returns empty if the range is out of range.
fn slice(data: &[u8], offset: u64, size: u64) -> &[u8] {
    let start = offset as usize;
    let end = start.saturating_add(size as usize);
    data.get(start..end).unwrap_or(&[])
}

/// Read a NUL-terminated name from a string table at `offset`.
fn str_at(strtab: &[u8], offset: u32) -> String {
    let start = offset as usize;
    if start >= strtab.len() {
        return String::new();
    }
    let rest = &strtab[start..];
    let end = rest.iter().position(|&b| b == 0).unwrap_or(rest.len());
    String::from_utf8_lossy(&rest[..end]).into_owned()
}