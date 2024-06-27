use std::{env::args, process::exit};

use lox_ast::{run_file, run_prompt};

fn main() {
    let args: Vec<String> = args().collect();

    match args.len() {
        1 => run_prompt(),
        2 => run_file(&args[1]).expect("Could not run file"),
        _ => {
            eprintln!("Usage: lax-ast [script]");
            exit(64);
        }
    }
}
