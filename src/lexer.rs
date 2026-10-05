use std::{iter::Peekable, str::Chars};

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Colon,
    String(String),
    Number(f64),
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

    fn string_token(&mut self) -> Token {
        let mut word = String::new();

        loop {
            match self.chars.peek() {
                Some(&'"') => {
                    self.chars.next();
                    break;
                }
                None => return Token::Illegal('"'),
                Some(&'\\') => {
                    self.chars.next();
                    if let Some(esc_ch) = self.chars.next() {
                        match esc_ch {
                            '"' => word.push('"'),
                            '\\' => word.push('\\'),
                            'n' => word.push('\n'),
                            't' => word.push('\t'),
                            'r' => word.push('\r'),
                            o => word.push(o),
                        }
                    }
                }
                Some(&c) => {
                    self.chars.next();
                    word.push(c);
                }
            }
        }

        Token::String(word)
    }

    fn number_token(&mut self) -> Token {
        let mut word = String::new();

        while let Some(&next_ch) = self.chars.peek() {
            if next_ch.is_numeric()
                || next_ch == '.'
                || next_ch == 'e'
                || next_ch == 'E'
                || next_ch == '-'
                || next_ch == '+'
            {
                self.chars.next();
                word.push(next_ch);
            } else {
                break;
            }
        }

        let real_number = word.parse().unwrap_or(0.0);

        Token::Number(real_number)
    }

    fn match_literal(&mut self, first_char: char) -> Token {
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
    type Item = Token;

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
            '"' => {
                self.chars.next();
                self.string_token()
            }
            '0'..='9' => self.number_token(),
            't' | 'f' | 'n' => self.match_literal(ch),
            c => Token::Illegal(c),
        };

        Some(token)
    }
}
