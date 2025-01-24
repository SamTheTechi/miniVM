pub fn parse_token(str: Vec<String>) -> Vec<i32> {
    let mut instructions: Vec<i32> = Vec::new();
    for s in str {
        if is_integer(&s) {
            instructions.push(str_to_int(&s));
        } else {
            let instruction: i32 = map_to_optcode(&s);
            if instruction != -1 {
                instructions.push(instruction);
            }
        }
    }
    instructions
}

fn map_to_optcode(str: &str) -> i32 {
    match str {
        "nop" => 0x40000000,
        "mov" => 0x40000001,
        "lod" => 0x40000002,
        "str" => 0x40000003,
        "jmp" => 0x40000004,
        "jml" => 0x40000005,
        "jmg" => 0x40000006,
        "jme" => 0x40000007,
        "rtn" => 0x40000008,
        "cmp" => 0x40000009,
        "out" => 0x4000000a,
        "sin" => 0x4000000b,
        "psh" => 0x4000000c,
        "pop" => 0x4000000d,
        "pek" => 0x4000000e,
        "swp" => 0x4000000f,
        "hlt" => 0x40000010,
        "int" => 0x40000011,
        "add" => 0x40000012,
        "sub" => 0x40000013,
        "mul" => 0x40000014,
        "div" => 0x40000015,
        "mod" => 0x40000016,
        "inc" => 0x40000017,
        "dec" => 0x40000018,
        "sqt" => 0x40000019,
        "and" => 0x4000001a,
        "sor" => 0x4000001b,
        "xor" => 0x4000001c,
        "not" => 0x4000001d,
        "shl" => 0x4000001e,
        "shr" => 0x4000001f,
        _ => -1,
    }
}

fn is_integer(str: &str) -> bool {
    for c in str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    true
}

fn str_to_int(str: &str) -> i32 {
    str.parse::<i32>()
        .unwrap_or_else(|_| panic!("invalid integer formant"))
}
