mod lexer;
mod parser;
mod translater;
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
    let (prog, heap) = parser::parse_token(token);

    match fs::File::create(&output_path) {
        Ok(_) => {}
        Err(e) => eprintln!("File to compilation {}:{}", output_path, e),
    }
    let byte_data = translater::into_binary(prog, heap);

    let _ = fs::write(output_path, byte_data);
}
