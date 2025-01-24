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
        "cmp" => 0x40000008,
        "clr" => 0x40000009,
        "cal" => 0x4000000a,
        "ret" => 0x4000000b,
        "swp" => 0x4000000c,
        "out" => 0x4000000d,
        "sin" => 0x4000000e,
        "psh" => 0x4000000f,
        "pop" => 0x40000010,
        "pek" => 0x40000011,
        "hlt" => 0x40000012,
        "add" => 0x40000013,
        "sub" => 0x40000014,
        "mul" => 0x40000015,
        "div" => 0x40000016,
        "mod" => 0x40000017,
        "inc" => 0x40000018,
        "dec" => 0x40000019,
        "sqt" => 0x4000001a,
        "and" => 0x4000001b,
        "sor" => 0x4000001c,
        "xor" => 0x4000001d,
        "not" => 0x4000001e,
        "shl" => 0x4000001f,
        "shr" => 0x40000020,
        // register
        "r0" => 0x40000022,
        "r1" => 0x40000023,
        "r2" => 0x40000024,
        "r3" => 0x40000025,
        "r4" => 0x40000026,
        "r5" => 0x40000027,
        "r6" => 0x40000028,
        "r7" => 0x40000029,
        "rf" => 0x4000002a,
        "rz" => 0x4000002b,
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
