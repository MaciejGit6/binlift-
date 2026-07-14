//! Per-format parsers.

pub mod elf;
pub mod pe;
pub mod macho;

use crate::error::Result;
use crate::model::Binary;

pub trait Parser{
    fn parse(data: &[u8]) -> Result<Binary>
    where
        Self: Sized;
}