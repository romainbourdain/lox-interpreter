use crate::error::LoxError;

use super::{
    token::{Object, Token},
    token_type::TokenType,
};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Scanner {
        Scanner {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, LoxError> {
        let mut had_error: Option<LoxError> = None;
        while !self.is_at_end() {
            self.start = self.current;
            if let Err(e) = self.scan_token() {
                e.report("".to_string());
                had_error = Some(e);
            }
        }

        self.tokens.push(Token::eof(self.line));

        if let Some(e) = had_error {
            Err(e)
        } else {
            Ok(&self.tokens)
        }
    }

    fn scan_token(&mut self) -> Result<(), LoxError> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '*' => self.add_token(TokenType::Star),
            '!' => self.add_token_if_else('=', TokenType::BangEqual, TokenType::Bang),
            '=' => self.add_token_if_else('=', TokenType::Equals, TokenType::Assign),
            '<' => self.add_token_if_else('=', TokenType::LessEqual, TokenType::Less),
            '>' => self.add_token_if_else('=', TokenType::GreaterEqual, TokenType::Greater),
            '/' => self.handle_slash()?,
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.scan_string()?,
            '0'..='9' => self.scan_number(),
            _ if c.is_ascii_alphanumeric() || c == '_' => self.scan_identifier(),
            _ => {
                return Err(LoxError::error(
                    self.line,
                    "unexpected character".to_string(),
                ))
            }
        }

        Ok(())
    }

    fn handle_slash(&mut self) -> Result<(), LoxError> {
        match self.peek() {
            Some('/') => {
                while !self.is_at_end() && self.peek() != Some('\n') {
                    self.advance();
                }
            }
            Some('*') => self.scan_comment_block()?,
            _ => self.add_token(TokenType::Slash),
        }

        Ok(())
    }

    fn scan_comment_block(&mut self) -> Result<(), LoxError> {
        loop {
            match self.peek() {
                Some('*') if self.peek_next() == Some('/') => {
                    self.advance();
                    self.advance();
                    return Ok(());
                }
                Some('/') if self.peek_next() == Some('*') => {
                    self.advance();
                    self.advance();
                    self.scan_comment_block()?;
                }
                Some('\n') => {
                    self.advance();
                    self.line += 1;
                }
                Some(_) => {
                    self.advance();
                }
                None => {
                    return Err(LoxError::error(
                        self.line,
                        "Unterminated comment".to_string(),
                    ))
                }
            }
        }
    }

    fn scan_identifier(&mut self) {
        while Scanner::is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        if let Some(t_type) = Scanner::keywords(&text) {
            self.add_token(t_type);
        } else {
            self.add_token(TokenType::Identifier);
        }
    }

    fn scan_number(&mut self) {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == Some('.') && Scanner::is_digit(self.peek_next()) {
            self.advance();

            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }
        let value: String = self.source[self.start..self.current].iter().collect();
        let num: f64 = value.parse().unwrap();
        self.add_token_object(TokenType::Number, Some(Object::Num(num)));
    }

    fn scan_string(&mut self) -> Result<(), LoxError> {
        while let Some(ch) = self.peek() {
            match ch {
                '"' => break,
                '\n' => self.line += 1,
                _ => {}
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(LoxError::error(
                self.line,
                "Undetermined string".to_string(),
            ));
        }

        self.advance();

        // TODO: handle escape sequence

        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();

        self.add_token_object(TokenType::String, Some(Object::Str(value)));
        Ok(())
    }
}

impl Scanner {
    fn advance(&mut self) -> char {
        let result = *self.source.get(self.current).unwrap();
        self.current += 1;
        result
    }

    fn add_token(&mut self, t_type: TokenType) {
        self.add_token_object(t_type, None);
    }

    fn add_token_object(&mut self, t_type: TokenType, literal: Option<Object>) {
        let lexeme = self.source[self.start..self.current].iter().collect();
        self.tokens
            .push(Token::new(t_type, lexeme, literal, self.line));
    }

    fn add_token_if_else(&mut self, expected: char, t_type: TokenType, else_type: TokenType) {
        if self.is_match(expected) {
            self.add_token(t_type);
        } else {
            self.add_token(else_type);
        }
    }

    fn is_match(&mut self, expected: char) -> bool {
        match self.source.get(self.current) {
            Some(ch) if *ch == expected => {
                self.current += 1;
                true
            }
            _ => false,
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.current).copied()
    }

    fn peek_next(&self) -> Option<char> {
        self.source.get(self.current + 1).copied()
    }

    fn keywords(check: &str) -> Option<TokenType> {
        match check {
            "and" => Some(TokenType::And),
            "class" => Some(TokenType::Class),
            "else" => Some(TokenType::Else),
            "false" => Some(TokenType::False),
            "for" => Some(TokenType::For),
            "fun" => Some(TokenType::Fun),
            "if" => Some(TokenType::If),
            "nil" => Some(TokenType::Nil),
            "or" => Some(TokenType::Or),
            "print" => Some(TokenType::Print),
            "return" => Some(TokenType::Return),
            "super" => Some(TokenType::Super),
            "this" => Some(TokenType::This),
            "true" => Some(TokenType::True),
            "var" => Some(TokenType::Var),
            "while" => Some(TokenType::While),
            _ => None,
        }
    }
}

impl Scanner {
    fn is_digit(ch: Option<char>) -> bool {
        if let Some(ch) = ch {
            ch.is_ascii_digit()
        } else {
            false
        }
    }

    fn is_alpha_numeric(ch: Option<char>) -> bool {
        if let Some(ch) = ch {
            ch.is_ascii_alphanumeric()
        } else {
            false
        }
    }
}
