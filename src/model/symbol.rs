
/// A named entity inside a binary — a function, variable, import, or export.
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub addr: u64,
    pub kind: SymbolKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    Function,
    Data,
    Import,
    Export,
    Unknown,
}