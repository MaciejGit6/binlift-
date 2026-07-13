//! binlift — format-agnostic binary loader (ELF / PE / Mach-O).

pub mod model;
pub mod loader;
pub mod formats;
pub mod binfmt;
pub mod error;
pub mod cli;
