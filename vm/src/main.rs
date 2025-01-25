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
        0xc0000001, // end
        0x4000000d, // out
        0x40000023, // r1
        0x40000012, // hlt
        0xc0000000, // start
        0x40000002, // lod
        0x40000022, // r0
        0x00000001, // 1
        0x40000002, // lod
        0x40000023, // r1
        0x00000200, // 1
        0x40000008, // cmp
        0x40000022, // r0
        0x40000023, // r1
        0x40000007, // jme
        0xc0000001, // end
        0x4000000d, // out
        0x40000023, // r1
        0x40000012, // hlt
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
