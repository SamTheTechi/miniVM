use asm::{is_integer, is_label, is_string, map_to_optcode, str_to_int};
use std::collections::HashMap;

pub fn parse_token(str: Vec<String>) -> (Vec<u32>, Vec<u8>) {
    let mut instructions: Vec<u32> = Vec::new();
    let mut string_pool: Vec<u8> = vec![0];
    let mut label: HashMap<String, u32> = HashMap::new();
    let mut current_address = 0xc0000001 as u32;
    label.insert("_start".to_string(), 0xc0000000);

    for s in str {
        if is_integer(&s) {
            instructions.push(str_to_int(&s));
        } else if is_string(&s) {
            string_pool.extend(s[1..s.len() - 1].bytes());
            string_pool.push(0);
            let val = (0x00ffffff & (string_pool.len() as i32 - s.len() as i32).max(0) as u32)
                | 0x80000000;
            instructions.push(val);
        } else if is_label(&s) {
            if let Some(&addr) = label.get(&s) {
                instructions.push(addr);
            } else {
                label.insert(s, current_address);
                instructions.push(current_address);
                current_address += 1;
            }
        } else {
            let instruction: u32 = map_to_optcode(&s);
            if instruction != 0xff {
                instructions.push(instruction | 0x40000000);
            }
        }
    }
    (instructions, string_pool)
}
