use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum ParseError {
    MissingFilePath,
    Io(std::io::Error),
    InvalidChar(char),
    UnterminatedString,
    UnexpectedToken { waiting: String, found: String },
    MissingColon,
    InvalidObjectKey,
    TrailingComma,
    UnexpectedEOF,
    UnknownError,
}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::Io(err)
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::MissingFilePath => write!(f, "Missing file path."),
            ParseError::Io(err) => write!(f, "IO Error: {err}."),
            ParseError::InvalidChar(c) => write!(f, "Invalid JSON char: '{c}'."),
            ParseError::UnterminatedString => write!(f, "Unterminated string: (string)."),
            ParseError::UnexpectedToken { waiting, found } => {
                write!(f, "Syntax error: waiting '{waiting}', found '{found}'.")
            }
            ParseError::MissingColon => write!(f, "Missing colon ':' after object key."),
            ParseError::InvalidObjectKey => write!(f, "Object key must be a string."),
            ParseError::TrailingComma => {
                write!(f, "JSON commas at the end of an object not allowed")
            }
            ParseError::UnexpectedEOF => write!(f, "Unexpected end of file."),
            ParseError::UnknownError => write!(f, "Unknown Error."),
        }
    }
}

impl Error for ParseError {}
