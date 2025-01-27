pub fn map_to_optcode(str: &str) -> u32 {
    match str {
        // instructions
        "nop" => 0x00,
        "mov" => 0x01,
        "lod" => 0x02,
        "str" => 0x03,
        "jmp" => 0x04,
        "jml" => 0x05,
        "jmg" => 0x06,
        "jme" => 0x07,
        "jmz" => 0x08,
        "cmp" => 0x09,
        "clr" => 0x0a,
        "cal" => 0x0b,
        "ret" => 0x0c,
        "swp" => 0x0d,
        "out" => 0x0e,
        "sin" => 0x0f,
        "nli" => 0x10,
        "psh" => 0x11,
        "pop" => 0x12,
        "pek" => 0x13,
        "hlt" => 0x14,
        "add" => 0x15,
        "sub" => 0x16,
        "mul" => 0x17,
        "div" => 0x18,
        "mod" => 0x19,
        "inc" => 0x1a,
        "dec" => 0x1b,
        "sqt" => 0x1c,
        "and" => 0x1d,
        "sor" => 0x1e,
        "xor" => 0x1f,
        "not" => 0x20,
        "shl" => 0x21,
        "shr" => 0x22,
        // register
        "r0" => 0xa5,
        "r1" => 0xa6,
        "r2" => 0xa7,
        "r3" => 0xa8,
        "r4" => 0xa9,
        "r5" => 0xaa,
        "r6" => 0xab,
        "r7" => 0xac,
        "rf" => 0xad,
        "rz" => 0xae,
        _ => 0xff,
    }
}

pub fn is_integer(str: &str) -> bool {
    str.starts_with('#')
        && (str[1..].chars().all(|c| c.is_digit(10))
            || str.chars().nth(1).unwrap() == '-' && str[2..].chars().all(|c| c.is_digit(10)))
}

pub fn is_string(s: &str) -> bool {
    s.starts_with("\"") && s.ends_with("\"")
}

pub fn is_label(s: &str) -> bool {
    s.starts_with('_') && s[1..].chars().all(|c| c.is_alphanumeric() || c == '_')
}

pub fn str_to_int(str: &str) -> u32 {
    if str.starts_with('#') {
        if str.chars().nth(1).unwrap() == '-' {
            str[2..].parse::<u32>().unwrap() | 0x20000000
        } else {
            str[1..].parse::<u32>().unwrap()
        }
    } else {
        panic!("Invalid integer format");
    }
}
