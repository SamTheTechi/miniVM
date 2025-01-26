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
            if instruction != 0x4fffffff {
                instructions.push(instruction);
            }
        }
    }
    (instructions, string_pool)
}

fn map_to_optcode(str: &str) -> u32 {
    match str {
        "nop" => 0x40000000,
        "mov" => 0x40000001,
        "lod" => 0x40000002,
        "str" => 0x40000003,
        "jmp" => 0x40000004,
        "jml" => 0x40000005,
        "jmg" => 0x40000006,
        "jme" => 0x40000007,
        "jmz" => 0x40000008,
        "cmp" => 0x40000009,
        "clr" => 0x4000000a,
        "cal" => 0x4000000b,
        "ret" => 0x4000000c,
        "swp" => 0x4000000d,
        "out" => 0x4000000e,
        "sin" => 0x4000000f,
        "nli" => 0x40000010,
        "psh" => 0x40000011,
        "pop" => 0x40000012,
        "pek" => 0x40000013,
        "hlt" => 0x40000014,
        "add" => 0x40000015,
        "sub" => 0x40000016,
        "mul" => 0x40000017,
        "div" => 0x40000018,
        "mod" => 0x40000019,
        "inc" => 0x4000001a,
        "dec" => 0x4000001b,
        "sqt" => 0x4000001c,
        "and" => 0x4000001d,
        "sor" => 0x4000001e,
        "xor" => 0x4000001f,
        "not" => 0x40000020,
        "shl" => 0x40000021,
        "shr" => 0x40000022,
        // register
        "r0" => 0x40000025,
        "r1" => 0x40000026,
        "r2" => 0x40000027,
        "r3" => 0x40000028,
        "r4" => 0x40000029,
        "r5" => 0x4000002a,
        "r6" => 0x4000002b,
        "r7" => 0x4000002c,
        "rf" => 0x4000002d,
        "rz" => 0x4000002e,
        _ => 0x4fffffff,
    }
}

fn is_integer(str: &str) -> bool {
    str.starts_with('#') && str[1..].chars().all(|c| c.is_digit(10))
}

fn is_string(s: &str) -> bool {
    s.starts_with("\"") && s.ends_with("\"")
}

fn is_label(s: &str) -> bool {
    s.starts_with('_') && s[1..].chars().all(|c| c.is_alphanumeric() || c == '_')
}

fn str_to_int(str: &str) -> u32 {
    str[1..]
        .parse::<u32>()
        .unwrap_or_else(|_| panic!("invalid integer formant"))
}
