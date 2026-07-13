
//! Checked byte-cursor reader.

use crate::error::{BinliftError, Result};

/// A cursor over a byte slice with bounds-checked reads.
pub struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Reader { data, pos: 0 }
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn seek(&mut self, pos: usize) {
        self.pos = pos;
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        let byte = *self
            .data
            .get(self.pos)
            .ok_or(BinliftError::Truncated(self.pos))?;
        self.pos += 1;
        Ok(byte)
    }

    pub fn read_u16_le(&mut self) -> Result<u16> {
        let bytes = self.take(2)?;
        Ok(u16::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_u32_le(&mut self) -> Result<u32> {
        let bytes = self.take(4)?;
        Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_u64_le(&mut self) -> Result<u64> {
        let bytes = self.take(8)?;
        Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
    }

    /// Borrow `n` bytes and advance, or fail if the slice is too short.
    fn take(&mut self, n: usize) -> Result<&'a [u8]> {
        let end = self.pos + n;
        let bytes = self
            .data
            .get(self.pos..end)
            .ok_or(BinliftError::Truncated(self.pos))?;
        self.pos = end;
        Ok(bytes)
    }
}