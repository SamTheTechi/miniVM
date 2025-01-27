pub fn decode_binary(content: Vec<u8>) -> (Vec<u32>, Vec<u8>) {
    let mut prog = Vec::new();
    let heap;
    let mut offset = 0;

    let prog_len = u32::from_le_bytes(content[offset..offset + 4].try_into().unwrap());
    offset += 4;

    for _ in 0..prog_len {
        prog.push(u32::from_le_bytes(
            content[offset..offset + 4].try_into().unwrap(),
        ));
        offset += 4;
    }

    let heap_len = u32::from_le_bytes(content[offset..offset + 4].try_into().unwrap());
    offset += 4;

    heap = content[offset..offset + heap_len as usize].to_vec();

    (prog, heap)
}
