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
        "hlt" => 0x40000001,
        "+" => 0x40000002,
        "-" => 0x40000003,
        "*" => 0x40000004,
        "/" => 0x40000005,
        "%" => 0x40000006,
        _ => -1,
    }
}

fn is_integer(str: &str) -> bool {
    for c in str.chars() {
        if !c.is_digit(10) {
            return false;
        }
    }
    true
}

fn str_to_int(str: &str) -> i32 {
    let n: i32 = str.parse().unwrap();
    n
}
