// Copyright 2024 Dimitrios Papakonstantinou. All rights reserved.
// Use of this source code is governed by a MIT
// license that can be found in the LICENSE file

// Types of language tokens

use std::fmt;

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

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::TokenVar => write!(f, "Token VAR"),
            TokenType::TokenEOF => write!(f, "Token EOF"),
            TokenType::TokenIdentifier => write!(f, "Token IDENTIFIER"),
            TokenType::TokenNumber => write!(f, "Token Number"),
            TokenType::TokenPlus => write!(f, "Token Plus"),
            TokenType::TokenMinus => write!(f, "Token Minus"),
            _ => todo!(),
        }
    }
}

pub struct Token {
    pub _type: TokenType,
    pub source_str: String,
    pub line: usize,
}

// Scanner is used to tokenize the source string
pub struct Scanner<'s> {
    pub source: &'s str,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub line: usize,
}

impl<'s> Scanner<'s> {
    // Instantiate The Scanner
    pub fn new(source: &'s str) -> Scanner<'s> {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    // Tokenize the source string and return a Token Vector
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        self.current = 0;

        // Tokenize the source string
        while !self.is_at_end() {
            self.scan_token();
        }

        // Append the 'End Of Line' Token
        self.tokens.push(Token {
            _type: TokenType::TokenEOF,
            source_str: "EOF".to_string(),
            line: self.line,
        });

        return &self.tokens;
    }

    // Scan each character and add the tokens to the Token vector
    fn scan_token(&mut self) -> () {
        self.skip_whitespace();
        self.start = self.current;

        match self.advance() {
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
            '*' => self.tokens.push(Token::new(TokenType::TokenStar, self)),
            ',' => self.tokens.push(Token::new(TokenType::TokenComma, self)),
            '+' => {
                // Check if its a two character token
                let res = if self.match_next('+') {
                    TokenType::TokenPlusPlus
                } else {
                    TokenType::TokenPlus
                };

                self.tokens.push(Token::new(res, self));
            }
            '=' => {
                // Check if its a two character token
                let res = if self.match_next('=') {
                    TokenType::TokenEqualEqual
                } else {
                    TokenType::TokenEqual
                };

                self.tokens.push(Token::new(res, self));
            }
            '!' => {
                // Check if its a two character token
                let res = if self.match_next('=') {
                    TokenType::TokenBangEqual
                } else {
                    TokenType::TokenBang
                };

                self.tokens.push(Token::new(res, self));
            }
            '<' => {
                // Check if its a two character token
                let res = if self.match_next('=') {
                    TokenType::TokenLessEqual
                } else {
                    TokenType::TokenLess
                };

                self.tokens.push(Token::new(res, self));
            }
            '>' => {
                // Check if its a two character token
                let res = if self.match_next('=') {
                    TokenType::TokenGreaterEqual
                } else {
                    TokenType::TokenGreater
                };

                self.tokens.push(Token::new(res, self));
            }
            '"' => {
                // String literal
                let res = self.lex_string();
                self.tokens.push(res);
            }
            c => {
                if c.is_numeric() {
                    self.lex_number();
                    return;
                } else if c.is_alphabetic() {
                    self.lex_identifier();
                    return;
                }

                self.tokens
                    .push(Token::error_token("Unexpected character".to_string(), self));
            }
        }
    }

    // Scan for identifier or keyword and add its type to the Token Vector
    fn lex_identifier(&mut self) -> () {
        while self.peek().is_alphabetic() || self.peek().is_numeric() {
            self.advance();
        }

        let value: &str = self.source[self.start..self.current].into();

        self.tokens
            .push(Token::new(self.match_keyword(value), self));
    }

    // Check if identifier is a keyword, return its type. If its not a keyword return identifier
    // type
    fn match_keyword(&self, word: &str) -> TokenType {
        match word {
            "and" => TokenType::TokenAnd,
            "class" => TokenType::TokenClass,
            "else" => TokenType::TokenElse,
            "false" => TokenType::TokenFalse,
            "time" => TokenType::TokenFor,
            "brick" => TokenType::TokenFun,
            "if" => TokenType::TokenIf,
            "nil" => TokenType::TokenNil,
            "or" => TokenType::TokenOr,
            "shine" => TokenType::TokenPrint,
            "return" => TokenType::TokenReturn,
            "true" => TokenType::TokenTrue,
            "pink" => TokenType::TokenVar,
            "echoes" => TokenType::TokenWhile,
            _ => TokenType::TokenIdentifier,
        }
    }

    // Scan number and add its type to the Token Vector
    fn lex_number(&mut self) -> () {
        while self.peek().is_numeric() && !self.is_at_end() {
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
        self.tokens.push(Token::new(TokenType::TokenNumber, self));
    }

    // Scan string and add its type to the Token Vector
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

    // Get current character. Get \0 if at the end
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        return self
            .source
            .chars()
            .nth(self.current)
            .unwrap_or_else(|| panic!("Error in peek(). No character at index {}", self.current));
    }

    // Get next character. Get \0 if the next character is at the end
    fn peek_next(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        return self
            .source
            .chars()
            .nth(self.current + 1)
            .unwrap_or_else(|| {
                panic!(
                    "Error in peek_next(). No character at index {}. Current character was {}",
                    self.current,
                    self.source.chars().nth(self.current).unwrap()
                )
            });
    }

    fn skip_whitespace(&mut self) -> () {
        while !self.is_at_end() {
            match self.peek() {
                ' ' | '\r' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                }
                '/' if self.peek_next() == '/' => {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }
                _ => return,
            }
        }
    }

    // Check if the next character is the expected character
    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.peek() != expected {
            return false;
        } else {
            self.current += 1;
            return true;
        }
    }

    // Check if scanner reached the end of source string
    fn is_at_end(&self) -> bool {
        return self.current == self.source.len();
    }

    // Get current char and continue to next character
    fn advance(&mut self) -> char {
        let char = self.peek();
        self.current += 1;
        char
    }
}

impl Token {
    // Create a new Token
    pub fn new(_type: TokenType, scanner: &Scanner) -> Token {
        let lexeme = &scanner.source[scanner.start..scanner.current];

        Token {
            _type,
            source_str: lexeme.to_string(),
            line: scanner.line,
        }
    }
    // Create an error Token, This type of Token has a msg as its source_str
    pub fn error_token(msg: String, scanner: &Scanner) -> Token {
        Token {
            _type: TokenType::TokenError,
            source_str: msg,
            line: scanner.line,
        }
    }
}
