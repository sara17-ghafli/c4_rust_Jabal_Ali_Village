//! c4.rs — C4 compiler rewritten in Rust
//! Author: Asma (First half)
//! ✅ Task: Rewrite the lexer component of the C4 compiler using Rust idioms

use std::str::Chars;
use std::iter::Peekable;

/// ✅ Represents the different kinds of tokens the lexer can produce
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Number(i32),
    Identifier(String),
    Keyword(String),
    Symbol(char),
    Eof,
}

/// ✅ Lexer struct to hold source input and current state
pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    /// ✅ Create a new lexer instance from input source code
    pub fn new(source: &'a str) -> Self {
        Lexer {
            input: source.chars().peekable(),
        }
    }

    /// ✅ Get the next token from input
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let ch = match self.input.peek() {
            Some(&c) => c,
            None => return Token::Eof,
        };

        if ch.is_ascii_digit() {
            return self.lex_number();
        }

        if ch.is_ascii_alphabetic() || ch == '_' {
            return self.lex_identifier_or_keyword();
        }

        // Handle symbols
        self.input.next();
        Token::Symbol(ch)
    }

    fn lex_number(&mut self) -> Token {
        let mut number = 0;
        while let Some(&ch) = self.input.peek() {
            if let Some(d) = ch.to_digit(10) {
                number = number * 10 + d as i32;
                self.input.next();
            } else {
                break;
            }
        }
        Token::Number(number)
    }

    fn lex_identifier_or_keyword(&mut self) -> Token {
        let mut ident = String::new();
        while let Some(&ch) = self.input.peek() {
            if ch.is_ascii_alphanumeric() || ch == '_' {
                ident.push(ch);
                self.input.next();
            } else {
                break;
            }
        }

        match ident.as_str() {
            "int" | "return" | "if" | "else" | "while" | "for" => Token::Keyword(ident),
            _ => Token::Identifier(ident),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(&ch) = self.input.peek() {
            if ch.is_whitespace() {
                self.input.next();
            } else {
                break;
            }
        }
    }
}

// 🧩 Asma's Part Ends Here
