use std::fs;

pub fn read_file_bits(path: &str) -> Option<Vec<u8>> {
    let bytes = fs::read(path).ok()?;
    let mut bits: Vec<u8> = Vec::new();

    for chunk in bytes.chunks(4) {
        if chunk.len() > 4 { break; }

        let value = u32::from_le_bytes([
            chunk[0], chunk[1], chunk[2], chunk[3]
        ]);

        for i in (0..32).rev() {
            bits.push(((value >> i) & 1) as u8);
        }
    }
    
    Some(bits)
}