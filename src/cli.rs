//! CLI args + output formatting.

use std::path::PathBuf;

use clap::Parser as ClapParser;

use crate::loader;

#[derive(ClapParser, Debug)]
#[command(name = "binlift", about = "Format-agnostic binary inspector (ELF/PE/Mach-O)")]
pub struct Cli {
    /// Path to the binary to inspect.
    pub path: PathBuf,

    /// Print the section table.
    #[arg(long)]
    pub sections: bool,

    /// Print the symbol table.
    #[arg(long)]
    pub symbols: bool,
}

pub fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    let data = std::fs::read(&cli.path)?;
    let binary = loader::load(&data)?;

    println!("format:      {:?}", binary.format);
    println!("entry point: {:#x}", binary.entry_point);
    println!("sections:    {}", binary.sections.len());
    println!("symbols:     {}", binary.symbols.len());

    if cli.sections {
        println!("\n{:<20} {:<14} {:>10}  perms", "name", "addr", "size");
        for s in &binary.sections {
            let p = format!(
                "{}{}{}",
                if s.perms.read { "r" } else { "-" },
                if s.perms.write { "w" } else { "-" },
                if s.perms.execute { "x" } else { "-" },
            );
            println!("{:<20} {:#012x} {:>10}  {}", s.name, s.addr, s.size, p);
        }
    }

    if cli.symbols {
        println!("\n{:<32} {:<14} kind", "name", "addr");
        for sym in &binary.symbols {
            println!("{:<32} {:#012x} {:?}", sym.name, sym.addr, sym.kind);
        }
    }
 
    Ok(())
}