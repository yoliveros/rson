use error::ParseError;
use lexer::Lexer;
use std::{fs, path::Path};

use crate::parser::Parser;

mod ast;
pub mod error;
mod lexer;
mod parser;

pub fn run<A: AsRef<Path>>(args: &[A]) -> Result<(), ParseError> {
    let path_arg = args.get(0).ok_or(ParseError::MissingFilePath)?;

    let file = fs::read_to_string(path_arg).map_err(|err| ParseError::Io(err))?;

    let lx = Lexer::new(&file);

    let json_value = Parser::new(lx);

    Ok(())
}
