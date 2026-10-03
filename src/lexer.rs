use std::{iter::Peekable, str::Chars};

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Colon,
    String(&'a str),
    Number(&'a str),
    True,
    False,
    Null,
    Illegal(char),
    EOF,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars().peekable(),
        }
    }

    fn string_token(&mut self) -> Token<'a> {
        Token::String("siu")
    }

    fn number_token(&mut self) -> Token<'a> {
        Token::Number("12345")
    }

    fn match_literal(&mut self, first_char: char) -> Token<'a> {
        let mut word = first_char.to_string();

        while let Some(&next_ch) = self.chars.peek() {
            if next_ch.is_alphabetic() {
                self.chars.next();
                word.push(next_ch);
            } else {
                break;
            }
        }

        match word.as_str() {
            "true" => Token::True,
            "false" => Token::False,
            "null" => Token::Null,
            _ => Token::Illegal(first_char),
        }
    }

    fn skip_white_space(&mut self) {
        while let Some(&ch) = self.chars.peek() {
            match ch {
                ' ' | '\t' | '\n' | '\r' => self.chars.next(),
                _ => break,
            };
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_white_space();

        let ch = match self.chars.next() {
            Some(c) => c,
            None => return Some(Token::EOF),
        };

        let token = match ch {
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            ':' => Token::Colon,
            ',' => Token::Comma,
            '"' => self.string_token(),
            't' | 'f' | 'n' => self.match_literal(ch),
            '0'..='9' => self.number_token(),
            c => Token::Illegal(c),
        };

        Some(token)
    }
}
