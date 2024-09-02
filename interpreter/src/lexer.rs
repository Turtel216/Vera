// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file

// Types of language tokens

use std::str::Chars;

// Types of language tokens
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum TokenType {
    // Single-character tokens.
    TokenRightParen,
    TokenLeftParen,
    TokenLeftBrace,
    TokenRightBrace,
    TokenComma,
    TokenDot,
    TokenMinus,
    TokenPlus,
    TokenSemicolon,
    TokenSlash,
    TokenStar,
    // One or two character tokens.
    TokenBang,
    TokenBangEqual,
    TokenEqual,
    TokenEqualEqual,
    TokenGreater,
    TokenGreaterEqual,
    TokenLess,
    TokenLessEqual,
    TokenPlusPlus,
    // Literals.
    TokenIdentifier,
    TokenString,
    TokenNumber,
    // Keywords.
    TokenAnd,
    TokenClass,
    TokenElse,
    TokenFalse,
    TokenFor,
    TokenFun,
    TokenIf,
    TokenNil,
    TokenOr,
    TokenPrint,
    TokenReturn,
    TokenTrue,
    TokenVar,
    TokenWhile,

    TokenError,
    TokenEOF,
}

struct Token {
    _type: TokenType,
    start: String,
    line: usize,
}

// Scanner is used to tokenize the source string
pub struct Scanner<'s> {
    source: &'s str,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl<'s> Scanner<'s> {
    pub fn new(source: &'s str) -> Scanner<'s> {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;

            self.scan_token();
        }

        self.tokens.push(Token {
            _type: TokenType::TokenEOF,
            start: "".to_string(),
            line: self.line,
        });

        return &self.tokens;
    }

    fn scan_token(&mut self) -> () {
        let current_char = self.advance();

        if current_char.is_numeric() {
            self.lex_number(current_char);
        }

        if current_char.is_alphabetic() {
            self.lex_identifier();
        }

        match current_char {
            '\n' => self.line += 1,
            ' ' | '\r' | '\t' => (),
            '(' => self
                .tokens
                .push(Token::new(TokenType::TokenLeftParen, self)),
            ')' => self
                .tokens
                .push(Token::new(TokenType::TokenRightParen, self)),
            '{' => self
                .tokens
                .push(Token::new(TokenType::TokenLeftBrace, self)),
            '}' => self
                .tokens
                .push(Token::new(TokenType::TokenRightBrace, self)),
            ';' => self
                .tokens
                .push(Token::new(TokenType::TokenSemicolon, self)),
            '.' => self.tokens.push(Token::new(TokenType::TokenDot, self)),
            '-' => self.tokens.push(Token::new(TokenType::TokenMinus, self)),
            '+' => self.tokens.push(Token::new(TokenType::TokenPlus, self)),
            '*' => self.tokens.push(Token::new(TokenType::TokenStar, self)),
            '=' => {
                let res = if self.match_next('=') {
                    TokenType::TokenEqualEqual
                } else {
                    TokenType::TokenEqual
                };

                self.tokens.push(Token::new(res, self));
            }
            '!' => {
                let res = if self.match_next('=') {
                    TokenType::TokenBangEqual
                } else {
                    TokenType::TokenBang
                };

                self.tokens.push(Token::new(res, self));
            }
            '<' => {
                let res = if self.match_next('=') {
                    TokenType::TokenLessEqual
                } else {
                    TokenType::TokenLess
                };

                self.tokens.push(Token::new(res, self));
            }
            '>' => {
                let res = if self.match_next('=') {
                    TokenType::TokenGreaterEqual
                } else {
                    TokenType::TokenGreater
                };

                self.tokens.push(Token::new(res, self));
            }
            '/' => {
                if self.peek_next() == '/' {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.tokens.push(Token::new(TokenType::TokenSlash, self));
                }
            }
            '"' => {
                let res = self.lex_string();
                self.tokens.push(res);
            }
            _ => !todo!(),
        }

        Token::error_token("Unexpected character.".to_string(), self);
    }

    fn lex_identifier(&mut self) -> () {
        while self.peek().is_alphabetic() || self.peek().is_numeric() {
            self.advance();
        }

        self.tokens.push(self.identifier_type());
    }

    fn identifier_type(&self) -> Token {
        match self.peek() {
            'p' => return self.check_keyword(1, 3, "ink", TokenType::TokenAnd),
            'b' => return self.check_keyword(1, 4, "rick", TokenType::TokenFun),
            's' => return self.check_keyword(1, 4, "hine", TokenType::TokenPrint),
            _ => Token::new(TokenType::TokenIdentifier, self),
        }
    }

    fn check_keyword(
        &self,
        _start: usize,
        _length: usize,
        _rest: &'s str,
        _type: TokenType,
    ) -> Token {
        if self.str_at_range(self.current + _start, _length) == _rest {
            return Token::new(_type, self);
        }
        return Token::new(TokenType::TokenIdentifier, self);
    }

    fn str_at_range(&self, start: usize, length: usize) -> &str {
        let sub_str = match self.source.get(start..start + length) {
            Some(s) => s,
            None => todo!("Handle case none"),
        };

        sub_str
    }

    fn lex_number(&mut self, current_char: char) -> Token {
        while current_char.is_numeric() {
            self.advance();
        }

        // Look for fractional part
        if self.peek() == '.' && self.peek_next().is_numeric() {
            // Consume .
            self.advance();

            while self.peek().is_numeric() {
                self.advance();
            }
        }
        return Token::new(TokenType::TokenNumber, self);
    }

    fn lex_string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Token::error_token("Unterminated string".to_string(), self);
        }

        self.advance();
        return Token::new(TokenType::TokenString, self);
    }

    fn peek(&self) -> char {
        return self.source.chars().nth(self.current).unwrap_or_else(|| {
            panic!(
                "Error in peek(). No character at index {}. Last character was {}",
                self.current,
                self.source.chars().nth(self.current - 1).unwrap()
            )
        });
    }

    fn peek_next(&self) -> char {
        return self
            .source
            .chars()
            .nth(self.current + 1)
            .unwrap_or_else(|| {
                panic!(
                    "Error in peek_next(). No character at index {}. Last character was {}",
                    self.current,
                    self.source.chars().nth(self.current - 1).unwrap()
                )
            });
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        let next_char = self.source.chars().nth(self.current).unwrap_or_else(|| {
            panic!(
                "No character at index {}. Last character was {}",
                self.current,
                self.source.chars().nth(self.current - 1).unwrap()
            )
        });

        return next_char != expected;
    }

    fn is_at_end(&self) -> bool {
        return self.current > self.source.len();
    }

    fn advance(&mut self) -> char {
        let char = self.source.chars().nth(self.current).unwrap_or_else(|| {
            println!(
                "No characters at index {} were found. Last character was {}.",
                self.current,
                self.source.chars().nth(self.current - 1).unwrap()
            );
            std::process::exit(1);
        });

        self.current += 1;

        return char;
    }
}

impl Token {
    pub fn new(_type: TokenType, scanner: &Scanner) -> Token {
        Token {
            _type,
            start: scanner.start.to_string(),
            line: scanner.line,
        }
    }
    pub fn error_token(msg: String, scanner: &Scanner) -> Token {
        Token {
            _type: TokenType::TokenError,
            start: msg,
            line: scanner.line,
        }
    }
}
