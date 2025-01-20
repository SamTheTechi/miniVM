pub struct VM {
    pc: i32,
    sp: i32,
    pub memory: Vec<i32>,
    typ: i32,
    dat: i32,
    pub running: i8,
}

impl VM {
    pub fn new() -> Self {
        VM {
            pc: 100,
            sp: 0,
            memory: vec![0; 1_000_000],
            typ: 0,
            dat: 0,
            running: 1,
        }
    }

    fn get_type(instruction: i32) -> i32 {
        let mut val: i32 = 0xc0000000u32 as i32;
        val = (val & instruction) >> 30;
        val
    }

    fn get_data(instruction: i32) -> i32 {
        let mut val: i32 = 0x3fffffff;
        val = val & instruction;
        val
    }

    fn fatch(&mut self) {
        self.pc = self.pc + 1;
    }
    fn decode(&mut self) {
        self.typ = Self::get_type(self.memory[self.pc as usize]);
        self.dat = Self::get_data(self.memory[self.pc as usize]);
    }
    fn execute(&mut self) {
        if self.typ == 0 || self.typ == 2 {
            self.sp += 1;
            self.memory[self.sp as usize] = self.dat;
            println!("push: {}", self.memory[self.sp as usize])
        } else {
            self.instruction_set();
        }
    }

    fn instruction_set(&mut self) {
        match self.dat {
            0 => {
                // nop
                println!("nop instruction");
            }
            1 => {
                // hlt
                println!("hlt");
                self.running = 0;
            }
            2 => {
                // add
                println!(
                    "add {} + {}",
                    self.memory[(self.sp - 1) as usize],
                    self.memory[self.sp as usize]
                );
                self.memory[self.sp as usize - 1] += self.memory[self.sp as usize];
                self.sp -= 1;
                println!("tos: {}", self.memory[self.sp as usize]);
            }
            3 => {
                // sub
                println!(
                    "sub {} - {}",
                    self.memory[(self.sp - 1) as usize],
                    self.memory[self.sp as usize]
                );
                self.memory[self.sp as usize - 1] -= self.memory[self.sp as usize];
                self.sp -= 1;
                println!("tos: {}", self.memory[self.sp as usize]);
            }
            4 => {
                // mul
                println!(
                    "mul {} * {}",
                    self.memory[(self.sp - 1) as usize],
                    self.memory[self.sp as usize]
                );
                self.memory[self.sp as usize - 1] *= self.memory[self.sp as usize];
                self.sp -= 1;
                println!("tos: {}", self.memory[self.sp as usize]);
            }
            5 => {
                // div
                println!(
                    "div {} / {}",
                    self.memory[(self.sp - 1) as usize],
                    self.memory[self.sp as usize]
                );
                self.memory[self.sp as usize - 1] /= self.memory[self.sp as usize];
                self.sp -= 1;
                println!("tos: {}", self.memory[self.sp as usize]);
            }
            _ => {
                println!("unknown instruction");
            }
        }
    }

    pub fn run(&mut self) {
        self.pc -= 1;
        while self.running == 1 {
            self.fatch();
            self.decode();
            self.execute();
        }
    }

    pub fn load_program(&mut self, prog: Vec<i32>) {
        for i in 0..prog.len() {
            self.memory[(self.pc as usize + i) as usize] = prog[i];
        }
    }
}
