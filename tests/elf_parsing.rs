//! ELF parsing integration tests.

use binlift::loader;
use binlift::model::BinaryFormat;

#[test]
fn parses_a_real_elf() {
    let data = std::fs::read("tests/fixtures/hello.elf")
        .expect("run: gcc -o tests/fixtures/hello.elf <a tiny C file>");
    let bin = loader::load(&data).unwrap();

    assert_eq!(bin.format, BinaryFormat::Elf);
    assert_ne!(bin.entry_point, 0);
    assert!(bin.sections.iter().any(|s| s.name == ".text"));
}