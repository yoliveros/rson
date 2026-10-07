use rson::run;
use std::process::exit;

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(ast) => {
            println!("JSON parsed!");
            println!("AST: {:#?}", ast);
        }
        Err(err) => {
            eprintln!("Error: {err}");
            exit(1);
        }
    }
}
