mod scanner;

use std::{env, fs, io};

use scanner::Scanner;

fn run(code: String) -> io::Result<()> {
    let scanner = Scanner::new(code);
    let tokens = scanner.scan_tokens();

    for tok in tokens {
        println!("{:?}", tok);
    }
    Ok(())
}

fn start_prompt() -> io::Result<()> {
    let mut line = String::new();
    loop {
        print!("> ");
        io::stdin().read_line(&mut line)?;
    }
}

fn run_file(filename: &String) -> io::Result<()> {
    let file = fs::read_to_string(filename)?;

    run(file)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0 => start_prompt(),
        1 => run_file(&args[0]),
        _ => {
            println!("Usage: rlox [script]");
            Ok(())
        }
    }
}
