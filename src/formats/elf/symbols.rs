//! ELF symbol parsing.

use crate::binfmt::Reader;
use crate::error::Result;
use crate::model::{Section, Symbol, SymbolKind};

const ELF64_SYM_SIZE: usize = 24;
const SHN_UNDEF: u16 = 0;
const STT_OBJECT: u8 = 1; // a data object (variable)
const STT_FUNC: u8 = 2; // a function

/// Extract symbols from `.symtab`, naming them via `.strtab`.
///
/// We locate both tables by their conventional names rather than by
/// following the symtab's `sh_link` field — good enough for typical
/// toolchain output, and a candidate for hardening later.
pub fn parse(sections: &[Section]) -> Result<Vec<Symbol>> {
    let symtab = match sections.iter().find(|s| s.name == ".symtab") {
        Some(s) => &s.data,
        None => return Ok(Vec::new()), // stripped binary — nothing to read
    };
    let strtab: &[u8] = sections
        .iter()
        .find(|s| s.name == ".strtab")
        .map(|s| s.data.as_slice())
        .unwrap_or(&[]);

    let count = symtab.len() / ELF64_SYM_SIZE;
    let mut symbols = Vec::with_capacity(count);

    for i in 0..count {
        let mut r = Reader::new(symtab);
        r.seek(i * ELF64_SYM_SIZE);

        let name_off = r.read_u32_le()?;
        let info = r.read_u8()?;
        let _other = r.read_u8()?;
        let shndx = r.read_u16_le()?;
        let value = r.read_u64_le()?;
        let _size = r.read_u64_le()?;

        let name = str_at(strtab, name_off);
        if name.is_empty() {
            continue; // skip the null symbol and unnamed section symbols
        }

        symbols.push(Symbol {
            name,
            addr: value,
            kind: classify(info, shndx),
        });
    }

    Ok(symbols)
}

fn classify(info: u8, shndx: u16) -> SymbolKind {
    if shndx == SHN_UNDEF {
        return SymbolKind::Import; // resolved from another object at link/load time
    }
    match info & 0xf {
        STT_FUNC => SymbolKind::Function,
        STT_OBJECT => SymbolKind::Data,
        _ => SymbolKind::Unknown,
    }
}

fn str_at(strtab: &[u8], offset: u32) -> String {
    let start = offset as usize;
    if start >= strtab.len() {
        return String::new();
    }
    let rest = &strtab[start..];
    let end = rest.iter().position(|&b| b == 0).unwrap_or(rest.len());
    String::from_utf8_lossy(&rest[..end]).into_owned()
}