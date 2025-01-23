mod asm;
mod lexer;
mod vm;
use lexer::Lexer;
use std::env;
use std::fs;
use std::process;
use vm::VM;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() < 2 {
        println!("cargo run <path.mth>");
        process::exit(0)
    }
    let content = fs::read_to_string(&arguments[1]).expect("something unexpected happend");
    let mut tokeniser = Lexer::new();
    let tokens = tokeniser.lexer(content);
    let prog = asm::parse_token(tokens.clone());
    let mut vm = VM::new();
    vm.load_program(prog);
    vm.run();
}
