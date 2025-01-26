use std::collections::HashMap;
use std::io;

const REG_CONST: u32 = 0x40000025;
pub struct VM {
    memory: Vec<u32>,
    stack: Vec<i32>,
    labels: HashMap<i32, i32>,
    heap: Vec<u8>,
    // gernal purpose registers
    regis: Vec<i32>,
    typ: u32,
    dat: u32,
    pc: i32,
    sp: usize,
    rning: bool,
}

// headers
// 0x0 -> positive   00
// 0x4 -> optcode    01
// 0x8 -> negative   10
// 0xc -> label      11
//

impl VM {
    pub fn new() -> Self {
        VM {
            memory: Vec::new(),
            stack: vec![0; 100],
            labels: HashMap::new(),
            heap: Vec::new(),
            regis: vec![0; 10],
            typ: 1,
            dat: 0,
            pc: 0,
            sp: 0,
            rning: true,
        }
    }

    fn get_type(instruction: u32) -> u32 {
        (instruction & 0xc0000000) >> 30
    }

    fn get_data(instruction: u32) -> u32 {
        instruction & 0x3fffffff
    }

    fn fetch(&mut self) {
        self.pc += 1;
    }

    fn decode(&mut self) {
        self.typ = Self::get_type(self.memory[self.pc as usize]);
        self.dat = Self::get_data(self.memory[self.pc as usize]);
    }

    fn execute(&mut self) {
        match self.typ {
            0x1 => {
                self.instruction_set();
            }
            0x3 => {}
            _ => {
                self.rning = false;
            }
        }
    }

    fn instruction_set(&mut self) {
        match self.dat {
            0 => {
                // nop
            }
            1 => {
                // mov
                if self.memory.len() as i32 > (self.pc + 2) {
                    let destination = (self.memory[(self.pc + 1) as usize] - REG_CONST) as usize;
                    self.regis[destination] = self.memory[(self.pc + 2) as usize] as i32;
                    self.pc += 2;
                }
            }
            2 => {
                // lod
                if self.memory.len() as i32 > (self.pc + 2) {
                    let destination = (self.memory[(self.pc + 1) as usize] - REG_CONST) as usize;
                    let source = (self.memory[(self.pc + 2) as usize] - REG_CONST) as usize;
                    self.regis[destination] = self.regis[source];
                    self.pc += 2;
                }
            }
            3 => {
                // str
                if self.memory.len() as i32 > (self.pc + 1) {
                    let destination = (self.memory[(self.pc + 1) as usize] - REG_CONST) as usize;
                    self.stack[self.sp] = self.regis[destination];
                    self.pc += 1;
                    self.sp += 1;
                }
            }
            4 => {
                // jmp
                if self.memory.len() as i32 > (self.pc + 1) {
                    let key = Self::get_data(self.memory[(self.pc + 1) as usize]);
                    self.pc = *self.labels.get(&(key as i32)).unwrap();
                }
            }
            5 => {
                // jml
                if self.memory.len() as i32 > (self.pc + 1) {
                    if self.regis[8] == 0 {
                        let key = Self::get_data(self.memory[(self.pc + 1) as usize]);
                        self.pc = *self.labels.get(&(key as i32)).unwrap();
                    } else {
                        self.pc += 1;
                    }
                }
            }
            6 => {
                // jmg
                if self.memory.len() as i32 > (self.pc + 1) {
                    if self.regis[8] == 1 {
                        let key = Self::get_data(self.memory[(self.pc + 1) as usize]);
                        self.pc = *self.labels.get(&(key as i32)).unwrap();
                    } else {
                        self.pc += 1;
                    }
                }
            }
            7 => {
                // jme
                if self.memory.len() as i32 > (self.pc + 1) {
                    if self.regis[9] == 1 {
                        let key = Self::get_data(self.memory[(self.pc + 1) as usize]);
                        self.pc = *self.labels.get(&(key as i32)).unwrap();
                    } else {
                        self.pc += 1;
                    }
                }
            }
            8 => {
                // jmz
                if self.memory.len() as i32 > (self.pc + 1) {
                    if self.regis[9] == 0 {
                        let key = Self::get_data(self.memory[(self.pc + 1) as usize]);
                        self.pc = *self.labels.get(&(key as i32)).unwrap();
                    } else {
                        self.pc += 1;
                    }
                }
            }
            9 => {
                // cmp
                if self.memory.len() as i32 > (self.pc + 2) {
                    let destination = (self.memory[(self.pc + 1) as usize] - REG_CONST) as usize;
                    let source = (self.memory[(self.pc + 2) as usize] - REG_CONST) as usize;
                    self.regis[9] = 0;
                    self.regis[8] = 0;
                    if self.regis[destination] == self.regis[source] {
                        self.regis[9] = 1;
                    } else if self.regis[destination] > self.regis[source] {
                        self.regis[8] = 1;
                    }
                    self.pc += 2;
                }
            }
            10 => {
                // clr
                if self.memory.len() as i32 > (self.pc + 1) {
                    let destination = (self.memory[(self.pc + 1) as usize] - REG_CONST) as usize;
                    self.regis[destination] = 0;
                    self.pc += 1;
                }
            }
            11 => {
                // cal
            }
            12 => {
                // ret
            }
            13 => {
                // swp
                if self.memory.len() as i32 > (self.pc + 2) {
                    let destination = (self.memory[(self.pc + 1) as usize] - REG_CONST) as usize;
                    let source = (self.memory[(self.pc + 2) as usize] - REG_CONST) as usize;
                    self.regis[source] = self.regis[source] + self.regis[destination];
                    self.regis[destination] = self.regis[source] + self.regis[destination];
                    self.regis[source] = self.regis[source] + self.regis[destination];
                    self.pc += 2;
                }
            }
            14 => {
                // out
                if self.memory.len() as i32 > (self.pc + 1) {
                    let source = (self.memory[(self.pc + 1) as usize] - REG_CONST) as usize;
                    print!("{}", self.regis[source]);
                    self.pc += 1;
                }
            }
            15 => {
                // sin
                if self.memory.len() as i32 > (self.pc + 1) {
                    let source = (self.memory[(self.pc + 1) as usize] - REG_CONST) as usize;
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("something went wrong");
                    self.regis[source] = input.trim().parse().unwrap();
                    self.pc += 1;
                }
            }
            16 => {
                // nli
                println!("");
            }
            17 => {
                // psh
                if self.memory.len() as i32 > (self.pc + 1) {
                    let source = (self.memory[(self.pc + 1) as usize] - REG_CONST) as usize;
                    self.stack[self.sp] = self.regis[source];
                    self.pc += 1;
                }
            }
            18 => {
                // pop
                if self.memory.len() as i32 > (self.pc + 1) {
                    let source = (self.memory[(self.pc + 1) as usize] - REG_CONST) as usize;
                    self.regis[source] = self.stack[self.sp];
                    self.sp -= 1;
                    self.pc += 1;
                }
            }
            19 => {
                // pek
                println!("{}", self.stack[self.sp]);
            }
            20 => {
                // hlt
                self.rning = false;
                println!("");
            }
            21 => {
                // add
                if self.memory.len() as i32 > (self.pc + 2) {
                    let destination = (self.memory[(self.pc + 1) as usize] - REG_CONST) as usize;
                    let source = (self.memory[(self.pc + 2) as usize] - REG_CONST) as usize;
                    self.regis[destination] += self.regis[source];
                    self.pc += 2;
                }
            }
            22 => {
                // sub
                if self.memory.len() as i32 > (self.pc + 2) {
                    let destination = (self.memory[(self.pc + 1) as usize] - REG_CONST) as usize;
                    let source = (self.memory[(self.pc + 2) as usize] - REG_CONST) as usize;
                    self.regis[destination] -= self.regis[source];
                    self.pc += 2;
                }
            }
            23 => {
                // mul
                if self.memory.len() as i32 > (self.pc + 2) {
                    let destination = (self.memory[(self.pc + 1) as usize] - REG_CONST) as usize;
                    let source = (self.memory[(self.pc + 2) as usize] - REG_CONST) as usize;
                    self.regis[destination] *= self.regis[source];
                    self.pc += 2;
                }
            }
            24 => {
                // div
                if self.memory.len() as i32 > (self.pc + 2) {
                    let destination = (self.memory[(self.pc + 1) as usize] - REG_CONST) as usize;
                    let source = (self.memory[(self.pc + 2) as usize] - REG_CONST) as usize;
                    self.regis[destination] /= self.regis[source];
                    self.pc += 2;
                }
            }
            25 => {
                // mod
                if self.memory.len() as i32 > (self.pc + 2) {
                    let destination = (self.memory[(self.pc + 1) as usize] - REG_CONST) as usize;
                    let source = (self.memory[(self.pc + 2) as usize] - REG_CONST) as usize;
                    self.regis[destination] %= self.regis[source];
                    self.pc += 2;
                }
            }
            26 => {
                // inc
                if self.memory.len() as i32 > (self.pc + 1) {
                    let source = (self.memory[(self.pc + 1) as usize] - REG_CONST) as usize;
                    self.regis[source] += 1;
                    self.pc += 1;
                }
            }
            27 => {
                // dec
                if self.memory.len() as i32 > (self.pc + 1) {
                    let source = (self.memory[(self.pc + 1) as usize] - REG_CONST) as usize;
                    self.regis[source] -= 1;
                    self.pc += 1;
                }
            }
            28 => {
                // sqt
            }
            29 => {
                // and
            }
            30 => {
                // sor
            }
            31 => {
                // xor
            }
            32 => {
                // not
            }
            33 => {
                // shl
            }
            34 => {
                // shr
            }
            _ => {}
        }
    }

    pub fn run(&mut self) {
        self.pc -= 1;
        while self.rning == true {
            self.fetch();
            self.decode();
            self.execute();
        }
    }
    pub fn load_program(&mut self, prog: Vec<u32>) {
        for (i, &instruction) in prog.iter().enumerate() {
            if Self::get_type(instruction) == 3 {
                self.labels
                    .entry(Self::get_data(instruction) as i32)
                    .or_insert(i as i32);
                if (Self::get_data(instruction) as i32) == 0 {
                    self.pc = (i + 1) as i32;
                }
            }
            self.memory.push(instruction);
        }
    }
}
