use std::{collections::HashMap, iter::Peekable};

use crate::{
    ast::JsonValue,
    error::ParseError,
    lexer::{Lexer, Token},
};

pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Parser {
            lexer: lexer.peekable(),
        }
    }

    pub fn parse_program(&mut self) -> Result<JsonValue, ParseError> {
        let value = self.parse_value()?;

        match self.lexer.next() {
            Some(Token::EOF) | None => Ok(value),
            Some(unex) => Err(ParseError::UnexpectedToken {
                waiting: "EOF".to_string(),
                found: format!("{:?}", unex),
            }),
        }
    }

    fn parse_value(&mut self) -> Result<JsonValue, ParseError> {
        let token = self.lexer.next().ok_or(ParseError::UnexpectedEOF)?;

        match token {
            Token::Null => Ok(JsonValue::Null),
            Token::True => Ok(JsonValue::Boolean(true)),
            Token::False => Ok(JsonValue::Boolean(false)),
            Token::Number(n) => Ok(JsonValue::Number(n)),
            Token::String(s) => Ok(JsonValue::String(s)),
            Token::LBrace => self.parse_object(),
            Token::LBracket => self.parse_array(),
            Token::Illegal(c) => Err(ParseError::InvalidChar(c)),
            _ => Err(ParseError::UnknownError),
        }
    }

    fn parse_array(&mut self) -> Result<JsonValue, ParseError> {
        let mut array = Vec::new();

        if self.lexer.peek() == Some(&Token::RBracket) {
            self.lexer.next();
            return Ok(JsonValue::Array(array));
        }

        loop {
            array.push(self.parse_value()?);

            match self.lexer.next() {
                Some(Token::Comma) => {
                    if self.lexer.peek() == Some(&Token::RBracket) {
                        return Err(ParseError::MissingColon);
                    }
                }
                Some(Token::RBracket) => break,
                _ => return Err(ParseError::UnknownError),
            };
        }

        Ok(JsonValue::Array(array))
    }

    fn parse_object(&mut self) -> Result<JsonValue, ParseError> {
        let mut map = HashMap::new();

        if self.lexer.peek() == Some(&Token::RBrace) {
            self.lexer.next();
            return Ok(JsonValue::Object(map));
        }

        loop {
            let key = match self.lexer.next() {
                Some(Token::String(s)) => s,
                _ => return Err(ParseError::InvalidObjectKey),
            };

            match self.lexer.next() {
                Some(Token::Colon) => {}
                _ => return Err(ParseError::MissingColon),
            }

            let value = self.parse_value()?;
            map.insert(key, value);

            match self.lexer.next() {
                Some(Token::Comma) => {
                    if self.lexer.peek() == Some(&Token::RBrace) {
                        return Err(ParseError::TrailingComma);
                    }
                }
                Some(Token::RBrace) => break,
                _ => return Err(ParseError::UnknownError),
            }
        }

        Ok(JsonValue::Object(map))
    }
}
