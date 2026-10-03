use rson::{error::ParseError, run};
use std::process::exit;

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if let Err(err) = run(&args) {
        match err {
            ParseError::MissingFilePath => eprintln!("Error: Missing file path."),
            ParseError::Io(e) => eprintln!("File read error: {e}"),
        }
        exit(1)
    }
}
