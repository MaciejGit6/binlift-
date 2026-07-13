//! Top-level normalized Binary type.

use crate::model::section::Section;
use crate::model::symbol::Symbol;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryFormat {
    Elf,
    Pe,
    MachO,
}

/// The format-agnostic description every parser produces.
#[derive(Debug, Clone)]
pub struct Binary {
    pub format: BinaryFormat,
    pub entry_point: u64,
    pub sections: Vec<Section>,
    pub symbols: Vec<Symbol>,
}