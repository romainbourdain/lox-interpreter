pub mod error;
pub mod expr;
pub mod lexer;

use std::{
    io::{self, stdout, BufRead, Write},
    process::exit,
};

use lexer::scanner::Scanner;

use error::LoxError;

pub fn run_file(path: &str) -> io::Result<()> {
    let buf = std::fs::read_to_string(path)?;
    if run(buf).is_err() {
        exit(65);
    }
    Ok(())
}
pub fn run_prompt() {
    let stdin = io::stdin();
    print!("> ");
    let _ = stdout().flush();
    for line in stdin.lock().lines() {
        if let Ok(line) = line {
            if line.is_empty() {
                break;
            }
            let _ = run(line.to_string());
        } else {
            break;
        }
        print!("> ");
        let _ = stdout().flush();
    }
}

fn run(source: String) -> Result<(), LoxError> {
    let mut scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{:?}", token);
    }

    Ok(())
}
