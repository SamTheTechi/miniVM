pub fn translater(prog: Vec<u32>) -> Vec<u8> {
    let mut byte_data = Vec::new();
    for &num in &prog {
        byte_data.extend_from_slice(&num.to_ne_bytes());
    }
    byte_data
}
