use std::collections::HashMap;

enum State {
    READVAL,
    READSTRING,
    READINTEGER,
    READLABEL,
    READREGIS,
    READINSTRUCTION,
    DUMP,
}

fn parse(str: Vec<String>) -> Vec<u32> {
    let mut instructions: Vec<u32> = Vec::new();
    let mut _string_pool: Vec<u8> = Vec::new();
    let mut _label: HashMap<String, u32> = HashMap::new();
    let mut state = State::READVAL;
    let mut counter = 0;
    let mut balance: i8 = 0;
    let mut buffer: u32 = 0;
    let length = str.len();

    println!("{}", length);
    while counter < length {
        match state {
            State::READVAL => {
                println!("{} balance {} buffer 0x{}", str[counter], balance, buffer);
                if is_integer(&str[counter]) && balance != 0 {
                    state = State::READINTEGER;
                } else if is_string(&str[counter]) && balance != 0 {
                    state = State::READSTRING;
                } else if is_register(&str[counter]) && balance != 0 {
                    state = State::READREGIS;
                } else if is_label(&str[counter]) {
                    state = State::READLABEL;
                } else {
                    state = State::READINSTRUCTION;
                }
            }
            State::READINTEGER => {
                counter += 1;
                println!("oops int");
                state = State::READVAL;
            }
            State::READLABEL => {
                counter += 1;
                println!("oops label");
                state = State::READVAL;
            }
            State::READREGIS => {
                let val = map_to_registers(&str[counter]);
                println!("{} balance {} buffer 0x{}", str[counter], balance, buffer);
                if val != 0xfff {
                    if balance == 1 {
                        buffer = buffer | (val & 0xfff);
                        balance -= 1;
                    } else if balance == 2 {
                        buffer = (buffer << 12) | (val & 0xfff);
                        balance -= 1;
                    }
                }
                if balance == 0 {
                    state = State::DUMP;
                }
                counter += 1;
            }
            State::READSTRING => {
                counter += 1;
                println!("oops string");
                state = State::READVAL;
            }
            State::READINSTRUCTION => {
                let ins: u32 = map_to_optcode(&str[counter]);
                if ins != 0xfff {
                    let ins_type = (ins & 0xff0) >> 4;
                    let ins_pram = ins & 0x00f;
                    if ins_pram != 0 {
                        balance += ins_pram as i8;
                        buffer = ins_type << 24;
                    } else {
                        instructions.push(ins_type << 24);
                    }
                    state = State::READVAL;
                } else {
                    state = State::READVAL;
                }
                counter += 1;
            }
            State::DUMP => {
                println!("Dumping buffer: 0x{:08x}", buffer);
                instructions.push(buffer);
                buffer = 0;
                balance = 0;
                state = State::READVAL;
            }
        }
    }
    instructions
}

pub fn map_to_registers(str: &str) -> u32 {
    match str {
        "r0" => 0x025,
        "r1" => 0x026,
        "r2" => 0x027,
        "r3" => 0x028,
        "r4" => 0x029,
        "r5" => 0x02a,
        "r6" => 0x02b,
        "r7" => 0x02c,
        "rf" => 0x02d,
        "rz" => 0x02e,
        "+int" => 0xa00,
        "-int" => 0xa01,
        "string" => 0xa02,
        "float" => 0xa03,
        _ => 0xfff,
    }
}

pub fn map_to_optcode(str: &str) -> u32 {
    match str {
        "nop" => 0x000,
        "mov" => 0x012,
        "lod" => 0x022,
        "str" => 0x032,
        "jmp" => 0x041,
        "jml" => 0x051,
        "jmg" => 0x061,
        "jme" => 0x071,
        "jmz" => 0x081,
        "cmp" => 0x092,
        "clr" => 0x0a1,
        "cal" => 0x0b0,
        "ret" => 0x0c0,
        "swp" => 0x0d2,
        "out" => 0x0e1,
        "sin" => 0x0f1,
        "nli" => 0x100,
        "psh" => 0x111,
        "pop" => 0x121,
        "pek" => 0x131,
        "hlt" => 0x140,
        "add" => 0x152,
        "sub" => 0x162,
        "mul" => 0x172,
        "div" => 0x182,
        "mod" => 0x192,
        "inc" => 0x1a1,
        "dec" => 0x1b1,
        "sqt" => 0x1c2,
        "and" => 0x1d2,
        "sor" => 0x1e2,
        "xor" => 0x1f2,
        "not" => 0x202,
        "shl" => 0x211,
        "shr" => 0x221,
        _ => 0xfff,
    }
}

fn is_hexval(val: u32) -> bool {
    (val & 0xfff000) == 0
}

pub fn is_register(reg: &str) -> bool {
    match reg {
        "r0" | "r1" | "r2" | "r3" | "r4" | "r5" | "r6" | "r7" | "rf" | "rz" => true,
        _ => false,
    }
}
