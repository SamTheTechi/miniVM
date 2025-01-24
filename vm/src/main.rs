mod vm;
use std::env;
use std::fs;
use std::process;
use vm::VM;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("cargo run <path.exe>");
        process::exit(0)
    }
    let mut vm = VM::new();
    let content = fs::read(&args[1]).expect("Something went wrong");

    let _prog: Vec<u32> = vec![
        0x00000002, // push 2
        0xc0000003, // push -8
        0x00000005, // push 1
        0x40000002, // add
        0xc0000023, // push -8
        0x00000003, // push 3
        0xc0000003, // push -8
        0x80000003, // push -8
        0x00000002, // push 2
        0x40000002, // add
        0xc0000003, // push -8
        0x00000002, // push 2
        0x40000001, //mov
        0x40000025, // r3
        0x40000029, // r7
        0x40000000, // halt
    ];

    let _progr: Vec<u32> = content
        .chunks(4)
        .filter_map(|chunk| {
            if chunk.len() == 4 {
                let val = u32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
                Some(val)
            } else {
                None
            }
        })
        .collect();
    vm.load_program(_prog);
    vm.run();
}
