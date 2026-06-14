use std::fs;

pub fn read_file_bits(path: &str) -> Option<Vec<u8>> {
    let bytes = fs::read(path).ok()?;
    Some(bytes)
}