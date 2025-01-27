pub fn into_binary(prog: Vec<u32>, heap: Vec<u8>) -> Vec<u8> {
    let mut binary = Vec::new();
    binary.extend_from_slice(&(prog.len() as u32).to_le_bytes());
    for &num in &prog {
        binary.extend_from_slice(&num.to_le_bytes());
    }
    binary.extend_from_slice(&(heap.len() as u32).to_le_bytes());
    for &chr in &heap {
        binary.push(chr);
    }
    binary
}
