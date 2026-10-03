use std::{fs, path::Path};

mod ast;
mod lexer;
mod parser;

pub fn run<A: AsRef<Path>>(args: &[A]) -> i32 {
    let path_arg = match args.get(0) {
        Some(path) => path,
        None => {
            eprintln!("Error: missing file path.");
            return 1;
        }
    };

    let file = match fs::read_to_string(path_arg) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error: {}", e);
            return 1;
        }
    };

    let parse = lexer::

    0
}
