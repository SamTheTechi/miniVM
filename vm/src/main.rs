mod decoder;
mod vm;
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("cargo run <path.exe>");
        process::exit(0)
    }
    let mut vm = vm::VM::new();
    let content = fs::read(&args[1]).expect("Something went wrong");

    let (prog, heap) = decoder::decode_binary(content);

    vm.load_program(prog, heap);
    vm.run();
}
