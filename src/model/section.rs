//! Normalized Section type.

#[derive(Debug, Clone)]
pub struct Section{
    pub name: String,
    pub addr: u64,
    pub size: u64,
    pub perms: SectionPerms,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct SectionPerms{
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

