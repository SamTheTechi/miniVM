use std::collections::HashMap;
use std::io;

pub struct VM {
    memory: Vec<u32>,
    stack: Vec<i32>,
    labels: HashMap<i32, i32>,
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
                    let destination = (self.memory[(self.pc + 1) as usize] - 0x40000022) as usize;
                    let source = (self.memory[(self.pc + 2) as usize] - 0x40000022) as usize;
                    self.regis[destination] = self.regis[source];
                    self.pc += 2;
                }
            }
            2 => {
                // lod
                if self.memory.len() as i32 > (self.pc + 2) {
                    let destination = (self.memory[(self.pc + 1) as usize] - 0x40000022) as usize;
                    self.regis[destination] = self.memory[(self.pc + 2) as usize] as i32;
                    self.pc += 2;
                }
            }
            3 => {
                // str
                if self.memory.len() as i32 > (self.pc + 1) {
                    let destination = (self.memory[(self.pc + 1) as usize] - 0x40000022) as usize;
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
                // cmp
                if self.memory.len() as i32 > (self.pc + 2) {
                    let destination = (self.memory[(self.pc + 1) as usize] - 0x40000022) as usize;
                    let source = (self.memory[(self.pc + 2) as usize] - 0x40000022) as usize;
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
            9 => {
                // clr
                if self.memory.len() as i32 > (self.pc + 1) {
                    let destination = (self.memory[(self.pc + 1) as usize] - 0x40000022) as usize;
                    self.regis[destination] = 0;
                    self.pc += 1;
                }
            }
            10 => {
                // cal
            }
            11 => {
                // ret
            }
            12 => {
                // swp
                if self.memory.len() as i32 > (self.pc + 2) {
                    let destination = (self.memory[(self.pc + 1) as usize] - 0x40000022) as usize;
                    let source = (self.memory[(self.pc + 2) as usize] - 0x40000022) as usize;
                    self.regis[source] = self.regis[source] + self.regis[destination];
                    self.regis[destination] = self.regis[source] + self.regis[destination];
                    self.regis[source] = self.regis[source] + self.regis[destination];
                    self.pc += 2;
                }
            }
            13 => {
                // out
                if self.memory.len() as i32 > (self.pc + 1) {
                    let source = (self.memory[(self.pc + 1) as usize] - 0x40000022) as usize;
                    println!("{}", self.regis[source]);
                    self.pc += 1;
                }
            }
            14 => {
                // sin
                if self.memory.len() as i32 > (self.pc + 1) {
                    let source = (self.memory[(self.pc + 1) as usize] - 0x40000022) as usize;
                    let mut input = String::new();
                    io::stdin()
                        .read_line(&mut input)
                        .expect("something went wrong");
                    self.regis[source] = input.trim().parse().unwrap();
                    self.pc += 1;
                }
            }
            15 => {
                // psh
                if self.memory.len() as i32 > (self.pc + 1) {
                    let source = (self.memory[(self.pc + 1) as usize] - 0x40000022) as usize;
                    self.stack[self.sp] = self.regis[source];
                    self.pc += 1;
                }
            }
            16 => {
                // pop
                if self.memory.len() as i32 > (self.pc + 1) {
                    let source = (self.memory[(self.pc + 1) as usize] - 0x40000022) as usize;
                    self.regis[source] = self.stack[self.sp];
                    self.sp -= 1;
                    self.pc += 1;
                }
            }
            17 => {
                // pek
                println!("{}", self.stack[self.sp]);
            }
            18 => {
                // hlt
                self.rning = false;
            }
            19 => {
                // add
                if self.memory.len() as i32 > (self.pc + 2) {
                    let destination = (self.memory[(self.pc + 1) as usize] - 0x40000022) as usize;
                    let source = (self.memory[(self.pc + 2) as usize] - 0x40000022) as usize;
                    self.regis[destination] += self.regis[source];
                    self.pc += 2;
                }
            }
            20 => {
                // sub
                if self.memory.len() as i32 > (self.pc + 2) {
                    let destination = (self.memory[(self.pc + 1) as usize] - 0x40000022) as usize;
                    let source = (self.memory[(self.pc + 2) as usize] - 0x40000022) as usize;
                    self.regis[destination] -= self.regis[source];
                    self.pc += 2;
                }
            }
            21 => {
                // mul
                if self.memory.len() as i32 > (self.pc + 2) {
                    let destination = (self.memory[(self.pc + 1) as usize] - 0x40000022) as usize;
                    let source = (self.memory[(self.pc + 2) as usize] - 0x40000022) as usize;
                    self.regis[destination] *= self.regis[source];
                    self.pc += 2;
                }
            }
            22 => {
                // div
                if self.memory.len() as i32 > (self.pc + 2) {
                    let destination = (self.memory[(self.pc + 1) as usize] - 0x40000022) as usize;
                    let source = (self.memory[(self.pc + 2) as usize] - 0x40000022) as usize;
                    self.regis[destination] /= self.regis[source];
                    self.pc += 2;
                }
            }
            23 => {
                // mod
                if self.memory.len() as i32 > (self.pc + 2) {
                    let destination = (self.memory[(self.pc + 1) as usize] - 0x40000022) as usize;
                    let source = (self.memory[(self.pc + 2) as usize] - 0x40000022) as usize;
                    self.regis[destination] %= self.regis[source];
                    self.pc += 2;
                }
            }
            24 => {
                // inc
                if self.memory.len() as i32 > (self.pc + 1) {
                    let source = (self.memory[(self.pc + 1) as usize] - 0x40000022) as usize;
                    self.regis[source] += 1;
                    self.pc += 1;
                }
            }
            25 => {
                // dec
                if self.memory.len() as i32 > (self.pc + 1) {
                    let source = (self.memory[(self.pc + 1) as usize] - 0x40000022) as usize;
                    self.regis[source] -= 1;
                    self.pc += 1;
                }
            }
            26 => {
                // sqt
            }
            27 => {
                // and
            }
            28 => {
                // sor
            }
            29 => {
                // xor
            }
            30 => {
                // not
            }
            31 => {
                // shl
            }
            32 => {
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
