pub struct VM {
    pc: i32,
    sp: i32,
    pub memory: Vec<i32>,
    typ: i32,
    dat: i32,
    pub running: i8,
}

impl VM {
    fn _fatch(&mut self) {
        self.pc = self.pc + 1;
    }
    fn _decode(&mut self) {
        self.typ = Self::get_type(self.memory[self.pc]);
        self.dat = Self::get_data(self.memory[self.pc]);
    }
    fn _execute(&mut self) {
        if self.typ == 0 || self.typ == 2 {
            self.sp = self.sp + 1;
        } else {
            Self::additional();
        }
    }

    fn get_type(instruction: i32) -> i32 {
        let mut val: i32 = 0xc0000000;
        val = (val & instruction) >> 30;
        val
    }
    fn get_data(instruction: i32) -> i32 {
        let mut val: i32 = 0x3fffffff;
        val = val & instruction;
        val
    }
    fn additional() {}

    pub fn _run() {
        println!("starting the VM !");
    }
    pub fn _load_program() {
        println!("starting the VM !");
    }
}
