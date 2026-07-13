use binlift::binfmt::Reader;

#[test]
fn reads_little_endian_u32() {
    let data = [0x01, 0x00, 0x00, 0x00];
    let mut r = Reader::new(&data);
    assert_eq!(r.read_u32_le().unwrap(), 1);
    assert_eq!(r.pos(), 4);
}

#[test]
fn reads_sequential_values() {
    let data = [0xAA, 0x01, 0x00];
    let mut r = Reader::new(&data);
    assert_eq!(r.read_u8().unwrap(), 0xAA);
    assert_eq!(r.read_u16_le().unwrap(), 1);
}

#[test]
fn truncated_read_errors_instead_of_panicking() {
    let data = [0x01, 0x02];
    let mut r = Reader::new(&data);
    assert!(r.read_u32_le().is_err());
}

#[test]
fn seek_repositions_cursor() {
    let data = [0x00, 0x00, 0x00, 0x00, 0x2A];
    let mut r = Reader::new(&data);
    r.seek(4);
    assert_eq!(r.read_u8().unwrap(), 42);
}