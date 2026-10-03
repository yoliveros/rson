#[derive(Debug)]
pub enum ParseError {
    MissingFilePath,
    Io(std::io::Error),
}

impl From<std::io::Error> for ParseError {
    fn from(err: std::io::Error) -> Self {
        ParseError::Io(err)
    }
}
