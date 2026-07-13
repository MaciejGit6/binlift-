//! Normalized, format-agnostic binary model.

pub mod binary;
pub mod section;
pub mod symbol;
pub mod entropy;

pub use binary::{Binary, BinaryFormat};
pub use section::{Section, SectionPerms};
pub use symbol::{Symbol, SymbolKind};