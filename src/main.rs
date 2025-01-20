mod vm;
use vm::VM;

fn main() {
    let mut vm = VM::new();
    let program: Vec<i32> = vec![
        0x00000008, // push 8
        0x00000005, // push 5
        0x00000002, // push 2
        0x40000002, // add
        0x00000003, // push 3
        0x40000004, // mul
        0x00000007, // push 7
        0x40000005, // div
        0x40000003, // sub
        0x40000001, // halt
    ];
    vm.load_program(program);
    vm.run();
}
