mod lexer;
mod parser;
use lexer::Lexer;
use std::env;
use std::fs;
use std::path;
use std::process;

fn main() {
    let mut tokeniser = Lexer::new();
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("cargo run <path.asm>");
        process::exit(0)
    }
    let input_path = &args[1];

    let output_path = match path::Path::new(input_path).with_extension("bin").to_str() {
        Some(name) => name.to_string(),
        None => {
            eprint!("Invalid filename: {}", input_path);
            process::exit(1);
        }
    };
    let content = fs::read_to_string(&input_path).expect("Something went wrong");
    let token = tokeniser.lexer(content);
    let prog = parser::parse_token(token);
    match fs::File::create(&output_path) {
        Ok(_) => println!("compilation success full! {}", output_path),
        Err(e) => eprintln!("File to compilation {}:{}", output_path, e),
    }
    let mut byte_data = Vec::new();
    for &num in &prog {
        byte_data.extend_from_slice(&num.to_ne_bytes());
    }
    let _ = fs::write(output_path, byte_data);
}
